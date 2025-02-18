use crate::color::Rgb;
use std::net::UdpSocket;

pub const WIDTH: u8 = 20;
pub const HEIGHT: u8 = 20;
pub const DEEPTH: u8 = 12;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Leds([[[Rgb; DEEPTH as usize]; HEIGHT as usize]; WIDTH as usize]);

impl Leds {
    const fn new(color: Rgb) -> Self {
        Self([[[color; DEEPTH as usize]; HEIGHT as usize]; WIDTH as usize])
    }

    fn read_at(&self, x: usize, y: usize, z: usize) -> Option<&Rgb> {
        self.0
            .get(x)
            .and_then(|ys| ys.get(y))
            .and_then(|zs| zs.get(z))
    }

    fn set_led(&mut self, x: usize, y: usize, z: usize, color: Rgb) {
        self.0[x][y][z] = color;
    }
}

pub trait Draw {
    fn color(&self) -> Rgb;
    fn draw(&self) -> Vec<(usize, usize, usize)>;
}

#[derive(Debug)]
pub enum VoxelBoxSendError {
    BindError,
    SendError,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Voxelbox {
    ip: String,
    port: u16,
    leds: Leds,
}

impl Voxelbox {
    pub fn new<T>(ip: T, port: u16) -> Self
    where
        T: Into<String>,
    {
        Self {
            ip: ip.into(),
            port,
            leds: Leds::new(Rgb::black()),
        }
    }

    pub fn reset_leds(&mut self) {
        self.leds = Leds::new(Rgb::black());
    }

    pub fn send(&self) -> Result<(), VoxelBoxSendError> {
        let socket = UdpSocket::bind("0.0.0.0:0").map_err(|_| VoxelBoxSendError::BindError)?;
        let destination = format!("{}:{}", self.ip, self.port);

        let mut data =
            Vec::with_capacity((WIDTH as usize) * (HEIGHT as usize) * (DEEPTH as usize) * 3);

        for z in 0..20 {
            for y in 0..20 {
                for x in 0..20 {
                    let (r, g, b) = self.leds.read_at(x, y, z).map_or((0, 0, 0), |&x| x.into());

                    data.push(r);
                    data.push(g);
                    data.push(b);
                }
            }
        }

        socket
            .send_to(&data, &destination)
            .map_err(|_| VoxelBoxSendError::SendError)?;
        Ok(())
    }

    pub fn draw<T>(&mut self, object: &T)
    where
        T: Draw,
    {
        let color = object.color();
        for (x, y, z) in object.draw() {
            self.leds.set_led(x, y, z, color);
        }
    }
}
