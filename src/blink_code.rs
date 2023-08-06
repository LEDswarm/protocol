/// A light sequence emitted by the controller to reflect different program states and errors.
///
/// The sequence varies in terms of colors and speed and provides a reliable way to display 
/// the events and states happening in the controller for debugging and user feedback.
#[derive(Debug, PartialEq, Clone)]
pub enum BlinkCode {
    WaitingForConnection,
    Connected,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlinkSequence {
    sequence: Vec<BlinkColor>,
    index: usize,
}

impl BlinkSequence {
    pub fn new() -> Self {
        Self {
            sequence: vec![],
            index: 0,
        }
    }

    pub fn iterate_color(&mut self) -> (u8, u8, u8, u8) {
        let (r, g, b, w) = self.sequence[self.index].rgbw;
        self.advance();

        (r, g, b, w)
    }

    fn advance(&mut self) {
        if self.index < self.sequence.len() - 1 {
            self.index += 1;
        } else {
            self.index = 0;
        }
    }

    pub fn from_code(code: BlinkCode) -> Self {
        let sequence = match code {
            BlinkCode::WaitingForConnection => vec![
                BlinkColor::short((0, 0, 255, 0)),
                BlinkColor::long((0, 0, 0, 0)),
            ],

            BlinkCode::Connected => vec![
                BlinkColor::long((0, 255, 0, 0)),
                BlinkColor::short((0, 0, 0, 0)),
            ],
        };

        return Self {
            sequence,
            index: 0,
        }
    }
}

/// To be used like:
///
/// ```rust
///   let mut sequence = BlinkSequence::from_code(BlinkCode::Connected);
///   for color in &mut blink_sequence {
///      // Use color.rgbw and color.duration
///   }
/// ```

impl Iterator for BlinkSequence {
    type Item = BlinkColor;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.sequence.get(self.index).cloned();
        self.advance();
        result
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BlinkColor {
    rgbw: (u8, u8, u8, u8),
    duration: BlinkDuration,
}

impl BlinkColor {
    pub fn short(rgbw: (u8, u8, u8, u8)) -> Self {
        Self {
            rgbw,
            duration: BlinkDuration::Short,
        }
    }

    pub fn long(rgbw: (u8, u8, u8, u8)) -> Self {
        Self {
            rgbw,
            duration: BlinkDuration::Short,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BlinkDuration {
    Short,
    Long,
}