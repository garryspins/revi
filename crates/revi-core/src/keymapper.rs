use crate::{mode::Mode, revi_command::ReViCommand};
use revi_ui::Key;
use std::collections::HashMap;

type KeyMap = HashMap<Vec<Key>, Vec<ReViCommand>>;

pub struct Mapper {
    nmaps: KeyMap,
    imaps: KeyMap,
    cmaps: KeyMap,
}

impl Default for Mapper {
    fn default() -> Self {
        Self::new().build_normal().build_insert().build_command()
    }
}

impl Mapper {
    fn new() -> Self {
        Self {
            nmaps: KeyMap::new(),
            imaps: KeyMap::new(),
            cmaps: KeyMap::new(),
        }
    }

    fn get_map(&self, mode: &Mode) -> &KeyMap {
        use Mode::*;
        match mode {
            Normal => &self.nmaps,
            Insert => &self.imaps,
            Command => &self.cmaps,
        }
    }

    fn get_map_mut(&mut self, mode: &Mode) -> &mut KeyMap {
        use Mode::*;
        match mode {
            Normal => &mut self.nmaps,
            Insert => &mut self.imaps,
            Command => &mut self.cmaps,
        }
    }

    pub fn get_mapping(&self, mode: &Mode, event: &[Key]) -> Option<&Vec<ReViCommand>> {
        self.get_map(mode).get(event)
    }

    pub fn insert_mapping(
        mut self,
        mode: &Mode,
        keys: Vec<Key>,
        commands: Vec<ReViCommand>,
    ) -> Self {
        self.get_map_mut(mode).insert(keys, commands);
        self
    }

    fn build_normal(self) -> Self {
        use Mode::*;
        self.insert_mapping(&Normal, vec![Key::Esc], vec![ReViCommand::Mode(Normal)])
            .insert_mapping(
                &Normal,
                vec![Key::UZ, Key::Shift, Key::UZ, Key::Shift],
                vec![ReViCommand::Save, ReViCommand::Quit],
            )
            .insert_mapping(
                &Normal,
                vec![Key::UZ, Key::Shift, Key::UQ, Key::Shift],
                vec![ReViCommand::Quit],
            )
            .insert_mapping(&Normal, vec![Key::LJ], vec![ReViCommand::CursorDown])
            .insert_mapping(&Normal, vec![Key::Down], vec![ReViCommand::CursorDown])
            .insert_mapping(&Normal, vec![Key::LK], vec![ReViCommand::CursorUp])
            .insert_mapping(&Normal, vec![Key::Up], vec![ReViCommand::CursorUp])
            .insert_mapping(&Normal, vec![Key::LH], vec![ReViCommand::CursorLeft])
            .insert_mapping(&Normal, vec![Key::Left], vec![ReViCommand::CursorLeft])
            .insert_mapping(&Normal, vec![Key::LL], vec![ReViCommand::CursorRight])
            .insert_mapping(&Normal, vec![Key::Right], vec![ReViCommand::CursorRight])
            .insert_mapping(&Normal, vec![Key::Colon], vec![ReViCommand::Mode(Command)])
            .insert_mapping(&Normal, vec![Key::LI], vec![ReViCommand::Mode(Insert)])
            .insert_mapping(&Normal, vec![Key::LX], vec![ReViCommand::DeleteChar])
            .insert_mapping(&Normal, vec![Key::Delete], vec![ReViCommand::DeleteChar])
            .insert_mapping(
                &Normal,
                vec![Key::LD, Key::LD],
                vec![ReViCommand::DeleteLine, ReViCommand::CursorUp],
            )
            .insert_mapping(&Normal, vec![Key::Home], vec![ReViCommand::Home])
            .insert_mapping(&Normal, vec![Key::End], vec![ReViCommand::End])
            .insert_mapping(&Normal, vec![Key::N0], vec![ReViCommand::Home])
            .insert_mapping(&Normal, vec![Key::Char('$')], vec![ReViCommand::End])
            .insert_mapping(
                &Normal,
                vec![Key::UA, Key::Shift],
                vec![
                    ReViCommand::End,
                    ReViCommand::Mode(Insert),
                    ReViCommand::CursorRight,
                ],
            )
            .insert_mapping(
                &Normal,
                vec![Key::LY, Key::Ctrl],
                vec![ReViCommand::ScrollUp, ReViCommand::CursorDown],
            )
            .insert_mapping(
                &Normal,
                vec![Key::LE, Key::Ctrl],
                vec![ReViCommand::ScrollDown, ReViCommand::CursorUp],
            )
            .insert_mapping(
                &Normal,
                vec![Key::LU, Key::Ctrl],
                vec![ReViCommand::ScrollUp],
            )
            .insert_mapping(
                &Normal,
                vec![Key::LD, Key::Ctrl],
                vec![ReViCommand::ScrollDown],
            )
            .insert_mapping(
                &Normal,
                vec![Key::LO],
                vec![
                    ReViCommand::End,
                    ReViCommand::Mode(Insert),
                    ReViCommand::CursorRight,
                    ReViCommand::NewLine,
                ],
            )
            .insert_mapping(
                &Normal,
                vec![Key::UO, Key::Shift],
                vec![
                    ReViCommand::Home,
                    ReViCommand::NewLine,
                    ReViCommand::Mode(Insert),
                    ReViCommand::CursorUp,
                ],
            )
            .insert_mapping(
                &Normal,
                vec![Key::Caret],
                vec![ReViCommand::FirstCharInLine],
            )
            .insert_mapping(
                &Normal,
                vec![Key::UI, Key::Shift],
                vec![ReViCommand::FirstCharInLine, ReViCommand::Mode(Insert)],
            )
            .insert_mapping(&Normal, vec![Key::LW], vec![ReViCommand::MoveForwardByWord])
            .insert_mapping(
                &Normal,
                vec![Key::LB],
                vec![ReViCommand::MoveBackwardByWord],
            )
            .insert_mapping(
                &Normal,
                vec![Key::LG, Key::LG],
                vec![ReViCommand::JumpToFirstLineBuffer],
            )
            .insert_mapping(
                &Normal,
                vec![Key::UG, Key::Shift],
                vec![ReViCommand::JumpToLastLineBuffer],
            )
    }

    fn build_insert(self) -> Self {
        use Mode::*;
        self.insert_mapping(&Insert, vec![Key::Esc], vec![ReViCommand::Mode(Normal)])
            .insert_mapping(&Insert, vec![Key::Backspace], vec![ReViCommand::Backspace])
            .insert_mapping(&Insert, vec![Key::Enter], vec![ReViCommand::NewLine])
            .insert_mapping(&Insert, vec![Key::Home], vec![ReViCommand::Home])
            .insert_mapping(&Insert, vec![Key::End], vec![ReViCommand::End])
            .insert_mapping(&Insert, vec![Key::Down], vec![ReViCommand::CursorDown])
            .insert_mapping(&Insert, vec![Key::Up], vec![ReViCommand::CursorUp])
            .insert_mapping(&Insert, vec![Key::Left], vec![ReViCommand::CursorLeft])
            .insert_mapping(&Insert, vec![Key::Right], vec![ReViCommand::CursorRight])
    }

    fn build_command(self) -> Self {
        use Mode::*;
        self.insert_mapping(&Command, vec![Key::Esc], vec![ReViCommand::Mode(Normal)])
            .insert_mapping(&Command, vec![Key::Enter], vec![ReViCommand::Mode(Normal)])
    }
}

/// Builds All Default Key
pub fn key_builder() -> Mapper {
    Mapper::new().build_normal().build_insert().build_command()
}
