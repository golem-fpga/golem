use embedded_graphics::pixelcolor::raw::ToBytes;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Rectangle;
use std::cell::RefCell;
use std::convert::{Infallible, TryFrom};
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

enum DrawBufferInner<C> {
    Empty,
    Buffer(Box<[C]>, Size),
    BufferRef(Rc<RefCell<DrawBufferInner<C>>>),
    SubBuffer(Rc<RefCell<DrawBufferInner<C>>>, Rectangle),
}

impl<C> Debug for DrawBufferInner<C> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Buffer(_, size) => f
                .debug_struct("DrawBuffer::Buffer")
                .field("size", size)
                .finish(),
            Self::SubBuffer(parent, rectangle) => f
                .debug_struct("DrawBuffer::SubBuffer")
                .field("rect", rectangle)
                .field("parent", parent)
                .finish(),
            DrawBufferInner::Empty => f.debug_struct("DrawBuffer::Empty").finish(),
            DrawBufferInner::BufferRef(parent) => f
                .debug_struct("DrawBuffer::BufferRef")
                .field("parent", parent)
                .finish(),
        }
    }
}

impl<C: PixelColor> DrawBufferInner<C> {
    pub fn with_default_color(size: Size, default_color: C) -> Self {
        Self::Buffer(
            vec![default_color; size.width as usize * size.height as usize].into_boxed_slice(),
            size,
        )
    }

    pub fn translate(&self, point: Point) -> Point {
        match self {
            DrawBufferInner::Buffer(_, _) => point,
            DrawBufferInner::SubBuffer(parent, rectangle) => {
                let parent_point = parent.borrow().translate(point);
                Point::new(
                    parent_point.x + rectangle.top_left.x,
                    parent_point.y + rectangle.top_left.y,
                )
            }
            DrawBufferInner::Empty => point,
            DrawBufferInner::BufferRef(parent) => parent.borrow().translate(point),
        }
    }

    /// Returns the color of the pixel at a point.
    pub fn get_pixel(&self, point: Point) -> C {
        match self {
            DrawBufferInner::Buffer(pixels, _) => {
                let index = self.point_to_index(point);

                pixels[index.expect("Point is outside the buffer size")]
            }
            DrawBufferInner::SubBuffer(parent, rectangle) => {
                let parent_point = point + rectangle.top_left;

                parent.borrow().get_pixel(parent_point)
            }
            DrawBufferInner::Empty => unreachable!("Empty buffer has no pixels"),
            DrawBufferInner::BufferRef(parent) => parent.borrow().get_pixel(point),
        }
    }

    pub fn set_pixel(&mut self, point: Point, color: C) {
        match self {
            DrawBufferInner::Buffer(buffer, size) => {
                let (x, y) = <(u32, u32)>::try_from(point).unwrap();
                if x < size.width && y < size.height {
                    buffer[(x + y * size.width) as usize] = color;
                }
            }
            DrawBufferInner::SubBuffer(parent, rectangle) => {
                let parent_point = point + rectangle.top_left;

                parent.borrow_mut().set_pixel(parent_point, color);
            }
            DrawBufferInner::Empty => {}
            DrawBufferInner::BufferRef(parent) => parent.borrow_mut().set_pixel(point, color),
        }
    }

    fn point_to_index(&self, point: Point) -> Option<usize> {
        if let DrawBufferInner::Buffer(_, size) = self {
            let (x, y) = <(u32, u32)>::try_from(point).ok()?;
            if x < size.width && y < size.height {
                return Some((x + y * size.width) as usize);
            }
        }

        None
    }
}

impl DrawBufferInner<BinaryColor> {
    pub fn invert(&mut self) {
        match self {
            DrawBufferInner::Empty => {}
            DrawBufferInner::Buffer(pixels, _) => {
                for pixel in pixels.iter_mut() {
                    *pixel = pixel.invert();
                }
            }
            DrawBufferInner::BufferRef(parent) => parent.borrow_mut().invert(),
            DrawBufferInner::SubBuffer(parent, rectangle) => {
                todo!()
            }
        }
    }
}

impl<C: PixelColor> DrawTarget for DrawBufferInner<C> {
    type Color = C;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        match self {
            DrawBufferInner::Buffer(buffer, size) => {
                for Pixel(point, color) in pixels.into_iter() {
                    let (x, y) = <(u32, u32)>::try_from(point).unwrap();
                    if x < size.width && y < size.height {
                        buffer[(x + y * size.width) as usize] = color;
                    }
                }
            }
            DrawBufferInner::SubBuffer(parent, rectangle) => {
                for Pixel(point, color) in pixels.into_iter() {
                    let parent_point = point + rectangle.top_left;

                    parent.borrow_mut().set_pixel(parent_point, color);
                }
            }
            DrawBufferInner::Empty => {}
            DrawBufferInner::BufferRef(parent) => {
                parent.borrow_mut().draw_iter(pixels)?;
            }
        }

        Ok(())
    }
}

impl<C: PixelColor> Drawable for DrawBufferInner<C> {
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        match self {
            DrawBufferInner::Buffer(pixels, Size { width, .. }) => {
                target.draw_iter(pixels.iter().enumerate().map(|(i, &c)| {
                    let x = (i as u32) % width;
                    let y = (i as u32) / width;
                    Pixel(Point::new(x as i32, y as i32), c)
                }))
            }
            DrawBufferInner::SubBuffer(parent, rect) => {
                unreachable!("Unimplemented: DrawBufferInner::SubBuffer::draw")
            }
            DrawBufferInner::Empty => Ok(()),
            DrawBufferInner::BufferRef(parent) => parent.borrow().draw(target),
        }
    }
}

impl<C> OriginDimensions for DrawBufferInner<C> {
    fn size(&self) -> Size {
        match self {
            DrawBufferInner::Buffer(_, size) => *size,
            DrawBufferInner::SubBuffer(_, rectangle) => rectangle.size,
            DrawBufferInner::Empty => Size::zero(),
            DrawBufferInner::BufferRef(parent) => parent.borrow().size(),
        }
    }
}

impl<C> DrawBufferInner<C>
where
    C: PixelColor + ToBytes,
    <C as ToBytes>::Bytes: AsRef<[u8]>,
{
    fn to_bytes<F>(&self, pixel_to_bytes: F) -> Vec<u8>
    where
        F: Fn(C) -> C::Bytes,
    {
        let mut bytes = Vec::new();

        match self {
            DrawBufferInner::Buffer(pixels, size) => {
                if C::Raw::BITS_PER_PIXEL >= 8 {
                    for pixel in pixels.iter() {
                        bytes.extend_from_slice(pixel_to_bytes(*pixel).as_ref())
                    }
                } else {
                    let pixels_per_byte = 8 / C::Raw::BITS_PER_PIXEL;

                    for row in pixels.chunks(size.width as usize) {
                        for byte_pixels in row.chunks(pixels_per_byte) {
                            let mut value = 0;

                            for pixel in byte_pixels {
                                value <<= C::Raw::BITS_PER_PIXEL;
                                value |= pixel.to_be_bytes().as_ref()[0];
                            }

                            value <<=
                                C::Raw::BITS_PER_PIXEL * (pixels_per_byte - byte_pixels.len());

                            bytes.push(value);
                        }
                    }
                }
            }
            DrawBufferInner::SubBuffer(_, _) => {
                unreachable!("Unsupported.");
            }
            DrawBufferInner::Empty => {}
            DrawBufferInner::BufferRef(parent) => {
                bytes.extend_from_slice(parent.borrow().to_bytes(pixel_to_bytes).as_ref())
            }
        }

        bytes
    }
}

/// A buffer that can be drawn to in EmbeddedDisplay. Most widgets will draw
/// directly to this.
#[derive(Debug)]
pub struct DrawBuffer<C> {
    inner: Rc<RefCell<DrawBufferInner<C>>>,
}

impl<C: PixelColor> DrawBuffer<C> {
    /// Creates a buffer filled with a color.
    pub fn with_default_color(size: Size, default_color: C) -> Self {
        Self {
            inner: Rc::new(RefCell::new(DrawBufferInner::with_default_color(
                size,
                default_color,
            ))),
        }
    }

    /// Returns the color of the pixel at a point.
    pub fn get_pixel(&self, point: Point) -> C {
        self.inner.borrow().get_pixel(point)
    }

    pub fn set_pixel(&mut self, point: Point, color: C) {
        self.inner.borrow_mut().set_pixel(point, color);
    }

    pub fn size(&self) -> Size {
        self.inner.borrow().size()
    }

    pub fn sub_buffer(&self, rectangle: Rectangle) -> Self {
        let intersection = rectangle.intersection(&self.bounding_box());

        if intersection.top_left == Point::zero() && intersection.size == self.size() {
            Self {
                inner: Rc::clone(&self.inner),
            }
        } else if intersection.is_zero_sized() {
            Self {
                inner: Rc::new(RefCell::new(DrawBufferInner::Empty)),
            }
        } else {
            Self {
                inner: Rc::new(RefCell::new(DrawBufferInner::SubBuffer(
                    Rc::clone(&self.inner),
                    intersection,
                ))),
            }
        }
    }
}

impl<C: PixelColor + From<BinaryColor>> DrawBuffer<C> {
    /// Creates a buffer filled with the equivalent of black (`BinaryColor::Off`).
    pub fn new(size: Size) -> Self {
        Self::with_default_color(size, C::from(BinaryColor::Off))
    }

    /// Creates a buffer that's the size of the OSD title in a MiSTer legacy setup.
    pub fn osd_title() -> Self {
        Self::new(Size::new(320, 16))
    }
}

impl DrawBuffer<BinaryColor> {
    pub fn invert(&mut self) {
        self.inner.borrow_mut().invert();
    }
}

impl<C> DrawBuffer<C>
where
    C: PixelColor + ToBytes,
    <C as ToBytes>::Bytes: AsRef<[u8]>,
{
    /// Converts the display content to big endian raw data.
    pub fn to_be_bytes(&self) -> Vec<u8> {
        self.inner.borrow().to_bytes(ToBytes::to_be_bytes)
    }

    /// Converts the display content to little endian raw data.
    pub fn to_le_bytes(&self) -> Vec<u8> {
        self.inner.borrow().to_bytes(ToBytes::to_le_bytes)
    }

    /// Converts the display content to native endian raw data.
    pub fn to_ne_bytes(&self) -> Vec<u8> {
        self.inner.borrow().to_bytes(ToBytes::to_ne_bytes)
    }
}

impl<C: PixelColor> DrawTarget for DrawBuffer<C> {
    type Color = C;
    type Error = Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.inner.borrow_mut().draw_iter(pixels)
    }
}

impl<C: PixelColor> Drawable for DrawBuffer<C> {
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        self.inner.borrow().draw(target)
    }
}

impl<C> OriginDimensions for DrawBuffer<C> {
    fn size(&self) -> Size {
        self.inner.borrow().size()
    }
}

#[test]
fn buffer_works() {
    let mut buffer = DrawBuffer::<BinaryColor>::new(Size::new(2, 2));
    buffer.set_pixel(Point::new(1, 1), BinaryColor::On);

    assert_eq!(buffer.get_pixel(Point::new(0, 0)), BinaryColor::Off);
    assert_eq!(buffer.get_pixel(Point::new(1, 1)), BinaryColor::On);

    assert_eq!(buffer.to_le_bytes(), b"\x00\x40");

    buffer.set_pixel(Point::new(0, 0), BinaryColor::On);
    assert_eq!(buffer.to_le_bytes(), b"\x80\x40");
}

#[test]
fn buffer_view() {
    let mut buffer = DrawBuffer::<BinaryColor>::new(Size::new(5, 5));
    buffer.set_pixel(Point::new(1, 1), BinaryColor::On);

    let view = buffer.sub_buffer(Rectangle::new(Point::new(1, 1), Size::new(2, 2)));
    assert_eq!(view.get_pixel(Point::new(0, 0)), BinaryColor::On);
}

#[test]
fn buffer_view_complex() {
    let mut buffer = DrawBuffer::<BinaryColor>::new(Size::new(100, 100));

    for x in 10..50 {
        for y in 10..50 {
            buffer.set_pixel(Point::new(x, y), BinaryColor::On);
        }
    }

    let view = buffer.sub_buffer(Rectangle::new(Point::new(20, 20), Size::new(20, 20)));
    let mut view2 = view.sub_buffer(Rectangle::new(Point::new(5, 5), Size::new(10, 10)));

    view2.set_pixel(Point::new(2, 2), BinaryColor::Off);

    assert_eq!(view2.get_pixel(Point::new(2, 2)), BinaryColor::Off);
    assert_eq!(view.get_pixel(Point::new(7, 7)), BinaryColor::Off);
    assert_eq!(buffer.get_pixel(Point::new(27, 27)), BinaryColor::Off);
}
