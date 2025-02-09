use crate::mode::Mode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReViCommand {
    StartUp,
    CursorUp,
    CursorDown,
    CursorLeft,
    CursorRight,
    ScrollUp,
    ScrollDown,
    Home,
    End,
    MoveForwardByWord,
    MoveBackwardByWord,
    JumpToFirstLineBuffer,
    JumpToLastLineBuffer,
    Backspace,
    NewLine,
    FirstCharInLine,
    DeleteChar,
    DeleteLine,
    InsertChar(char),
    Mode(Mode),
    Save,
    Quit,
}
