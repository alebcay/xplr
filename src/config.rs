use crate::app::ExternalMsg;
use crate::app::HelpMenuLine;
use crate::app::NodeFilter;
use crate::app::NodeSorter;
use crate::app::NodeSorterApplicable;
use crate::default_config;
use crate::ui::Style;
use anyhow::Result;
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::HashMap;
use tui::layout::Constraint as TuiConstraint;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Action {
    #[serde(default)]
    pub help: Option<String>,

    #[serde(default)]
    pub messages: Vec<ExternalMsg>,
}

impl Action {
    pub fn sanitized(self, read_only: bool) -> Option<Self> {
        if self.messages.is_empty() {
            None
        } else if read_only {
            if self.messages.iter().all(|m| m.is_read_only()) {
                Some(self)
            } else {
                None
            }
        } else {
            Some(self)
        }
    }

    pub fn extend(mut self, other: Self) -> Self {
        self.help = other.help.or(self.help);
        self.messages = other.messages;
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeTypeConfig {
    #[serde(default)]
    pub style: Style,

    #[serde(default)]
    pub meta: HashMap<String, String>,
}

impl NodeTypeConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.style = self.style.extend(other.style);
        self.meta.extend(other.meta);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NodeTypesConfig {
    #[serde(default)]
    pub directory: NodeTypeConfig,

    #[serde(default)]
    pub file: NodeTypeConfig,

    #[serde(default)]
    pub symlink: NodeTypeConfig,

    #[serde(default)]
    pub mime_essence: HashMap<String, NodeTypeConfig>,

    #[serde(default)]
    pub extension: HashMap<String, NodeTypeConfig>,

    #[serde(default)]
    pub special: HashMap<String, NodeTypeConfig>,
}

impl NodeTypesConfig {
    fn extend(mut self, other: Self) -> Self {
        self.directory = self.directory.extend(other.directory);
        self.file = self.file.extend(other.file);
        self.symlink = self.symlink.extend(other.symlink);
        self.mime_essence.extend(other.mime_essence);
        self.extension.extend(other.extension);
        self.special.extend(other.special);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiConfig {
    #[serde(default)]
    pub prefix: Option<String>,

    #[serde(default)]
    pub suffix: Option<String>,

    #[serde(default)]
    pub style: Style,
}

impl UiConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.prefix = other.prefix.or(self.prefix);
        self.suffix = other.suffix.or(self.suffix);
        self.style = self.style.extend(other.style);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiElement {
    #[serde(default)]
    pub format: Option<String>,

    #[serde(default)]
    pub style: Style,
}

impl UiElement {
    fn extend(mut self, other: Self) -> Self {
        self.format = other.format.or(self.format);
        self.style = self.style.extend(other.style);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableRowConfig {
    #[serde(default)]
    pub cols: Option<Vec<UiElement>>,

    #[serde(default)]
    pub style: Style,

    #[serde(default)]
    pub height: Option<u16>,
}

impl TableRowConfig {
    fn extend(mut self, other: Self) -> Self {
        self.cols = other.cols.or(self.cols);
        self.style = self.style.extend(other.style);
        self.height = other.height.or(self.height);
        self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Constraint {
    Percentage(u16),
    Ratio(u32, u32),
    Length(u16),
    Max(u16),
    Min(u16),
}

impl Default for Constraint {
    fn default() -> Self {
        Self::Min(1)
    }
}

impl Into<TuiConstraint> for Constraint {
    fn into(self) -> TuiConstraint {
        match self {
            Self::Length(n) => TuiConstraint::Length(n),
            Self::Percentage(n) => TuiConstraint::Percentage(n),
            Self::Ratio(x, y) => TuiConstraint::Ratio(x, y),
            Self::Max(n) => TuiConstraint::Max(n),
            Self::Min(n) => TuiConstraint::Min(n),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableConfig {
    #[serde(default)]
    pub header: TableRowConfig,

    #[serde(default)]
    pub row: TableRowConfig,

    #[serde(default)]
    pub style: Style,

    #[serde(default)]
    pub tree: Option<(UiElement, UiElement, UiElement)>,

    #[serde(default)]
    pub col_spacing: Option<u16>,

    #[serde(default)]
    pub col_widths: Option<Vec<Constraint>>,
}

impl TableConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.header = self.header.extend(other.header);
        self.row = self.row.extend(other.row);
        self.style = self.style.extend(other.style);
        self.tree = other.tree.or(self.tree);
        self.col_spacing = other.col_spacing.or(self.col_spacing);
        self.col_widths = other.col_widths.or(self.col_widths);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LogsConfig {
    #[serde(default)]
    pub info: UiElement,

    #[serde(default)]
    pub success: UiElement,

    #[serde(default)]
    pub error: UiElement,
}

impl LogsConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.info = self.info.extend(other.info);
        self.success = self.success.extend(other.success);
        self.error = self.error.extend(other.error);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SortDirectionIdentifiersUi {
    #[serde(default)]
    pub forward: UiElement,

    #[serde(default)]
    pub reverse: UiElement,
}

impl SortDirectionIdentifiersUi {
    pub fn extend(mut self, other: Self) -> Self {
        self.forward = self.forward.extend(other.forward);
        self.reverse = self.reverse.extend(other.reverse);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SortAndFilterUi {
    #[serde(default)]
    pub separator: UiElement,

    #[serde(default)]
    pub sort_direction_identifiers: SortDirectionIdentifiersUi,

    #[serde(default)]
    pub sorter_identifiers: HashMap<NodeSorter, UiElement>,

    #[serde(default)]
    pub filter_identifiers: HashMap<NodeFilter, UiElement>,
}

impl SortAndFilterUi {
    pub fn extend(mut self, other: Self) -> Self {
        self.separator = self.separator.extend(other.separator);
        self.sort_direction_identifiers = self
            .sort_direction_identifiers
            .extend(other.sort_direction_identifiers);
        self.sorter_identifiers.extend(other.sorter_identifiers);
        self.filter_identifiers.extend(other.filter_identifiers);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeneralConfig {
    #[serde(default)]
    pub show_hidden: Option<bool>,

    #[serde(default)]
    pub read_only: Option<bool>,

    #[serde(default)]
    pub cursor: UiElement,

    #[serde(default)]
    pub prompt: UiElement,

    #[serde(default)]
    pub logs: LogsConfig,

    #[serde(default)]
    pub table: TableConfig,

    #[serde(default)]
    pub default_ui: UiConfig,

    #[serde(default)]
    pub focus_ui: UiConfig,

    #[serde(default)]
    pub selection_ui: UiConfig,

    #[serde(default)]
    pub sort_and_filter_ui: SortAndFilterUi,

    #[serde(default)]
    pub initial_sorting: Option<IndexSet<NodeSorterApplicable>>,
}

impl GeneralConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.show_hidden = other.show_hidden.or(self.show_hidden);
        self.read_only = other.read_only.or(self.read_only);
        self.cursor = self.cursor.extend(other.cursor);
        self.prompt = self.prompt.extend(other.prompt);
        self.logs = self.logs.extend(other.logs);
        self.table = self.table.extend(other.table);
        self.default_ui = self.default_ui.extend(other.default_ui);
        self.focus_ui = self.focus_ui.extend(other.focus_ui);
        self.selection_ui = self.selection_ui.extend(other.selection_ui);
        self.sort_and_filter_ui = self.sort_and_filter_ui.extend(other.sort_and_filter_ui);
        self.initial_sorting = other.initial_sorting.or(self.initial_sorting);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KeyBindings {
    #[serde(default)]
    pub remaps: BTreeMap<String, String>,

    #[serde(default)]
    pub on_key: BTreeMap<String, Action>,

    #[serde(default)]
    pub on_alphabet: Option<Action>,

    #[serde(default)]
    pub on_number: Option<Action>,

    #[serde(default)]
    pub on_special_character: Option<Action>,

    #[serde(default)]
    pub default: Option<Action>,
}

impl KeyBindings {
    pub fn sanitized(mut self, read_only: bool) -> Self {
        if read_only {
            self.on_key = self
                .on_key
                .into_iter()
                .filter_map(|(k, a)| a.sanitized(read_only).map(|a| (k, a)))
                .collect();

            self.on_alphabet = self.on_alphabet.and_then(|a| a.sanitized(read_only));
            self.on_number = self.on_number.and_then(|a| a.sanitized(read_only));
            self.on_special_character = self
                .on_special_character
                .and_then(|a| a.sanitized(read_only));
            self.default = self.default.and_then(|a| a.sanitized(read_only));
            self.remaps = self
                .remaps
                .clone()
                .into_iter()
                .filter(|(_, v)| self.on_key.contains_key(v))
                .collect();
            self
        } else {
            self
        }
    }

    pub fn extend(mut self, other: Self) -> Self {
        self.remaps.extend(other.remaps);
        self.on_key.extend(other.on_key);
        self.on_alphabet = other.on_alphabet.or(self.on_alphabet);
        self.on_number = other.on_number.or(self.on_number);
        self.on_special_character = other.on_special_character.or(self.on_special_character);
        self.default = other.default.or(self.default);
        self
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Mode {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub help: Option<String>,

    #[serde(default)]
    pub extra_help: Option<String>,

    #[serde(default)]
    pub key_bindings: KeyBindings,
}

impl Mode {
    pub fn sanitized(mut self, read_only: bool) -> Self {
        self.key_bindings = self.key_bindings.sanitized(read_only);
        self
    }

    pub fn extend(mut self, other: Self) -> Self {
        self.help = other.help.or(self.help);
        self.extra_help = other.extra_help.or(self.extra_help);
        self.key_bindings = self.key_bindings.extend(other.key_bindings);
        self
    }

    pub fn help_menu(&self) -> Vec<HelpMenuLine> {
        let extra_help_lines = self.extra_help.clone().map(|e| {
            e.lines()
                .map(|l| HelpMenuLine::Paragraph(l.into()))
                .collect::<Vec<HelpMenuLine>>()
        });

        self.help
            .clone()
            .map(|h| {
                h.lines()
                    .map(|l| HelpMenuLine::Paragraph(l.into()))
                    .collect()
            })
            .unwrap_or_else(|| {
                extra_help_lines
                    .unwrap_or_default()
                    .into_iter()
                    .chain(
                        self.key_bindings
                            .on_key
                            .iter()
                            .filter(|(k, _)| !self.key_bindings.remaps.contains_key(&k.to_string()))
                            .filter_map(|(k, a)| {
                                a.help.clone().map(|h| HelpMenuLine::KeyMap(k.into(), h))
                            }),
                    )
                    .chain(
                        self.key_bindings
                            .on_alphabet
                            .iter()
                            .map(|a| ("[a-Z]", a.help.clone()))
                            .filter_map(|(k, mh)| mh.map(|h| HelpMenuLine::KeyMap(k.into(), h))),
                    )
                    .chain(
                        self.key_bindings
                            .on_number
                            .iter()
                            .map(|a| ("[0-9]", a.help.clone()))
                            .filter_map(|(k, mh)| mh.map(|h| HelpMenuLine::KeyMap(k.into(), h))),
                    )
                    .chain(
                        self.key_bindings
                            .on_special_character
                            .iter()
                            .map(|a| ("[spcl chars]", a.help.clone()))
                            .filter_map(|(k, mh)| mh.map(|h| HelpMenuLine::KeyMap(k.into(), h))),
                    )
                    .chain(
                        self.key_bindings
                            .default
                            .iter()
                            .map(|a| ("[default]", a.help.clone()))
                            .filter_map(|(k, mh)| mh.map(|h| HelpMenuLine::KeyMap(k.into(), h))),
                    )
                    .collect()
            })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuiltinModesConfig {
    #[serde(default)]
    pub default: Mode,

    #[serde(default)]
    pub selection_ops: Mode,

    #[serde(default)]
    pub create: Mode,

    #[serde(default)]
    pub create_directory: Mode,

    #[serde(default)]
    pub create_file: Mode,

    #[serde(default)]
    pub number: Mode,

    #[serde(default)]
    pub go_to: Mode,

    #[serde(default)]
    pub rename: Mode,

    #[serde(default)]
    pub delete: Mode,

    #[serde(default)]
    pub action: Mode,

    #[serde(default)]
    pub search: Mode,

    #[serde(default)]
    pub filter: Mode,

    #[serde(default)]
    pub relative_path_does_contain: Mode,

    #[serde(default)]
    pub relative_path_does_not_contain: Mode,

    #[serde(default)]
    pub sort: Mode,
}

impl BuiltinModesConfig {
    pub fn extend(mut self, other: Self) -> Self {
        self.default = self.default.extend(other.default);
        self.selection_ops = self.selection_ops.extend(other.selection_ops);
        self.go_to = self.go_to.extend(other.go_to);
        self.create = self.create.extend(other.create);
        self.create_file = self.create_file.extend(other.create_file);
        self.create_directory = self.create_directory.extend(other.create_directory);
        self.rename = self.rename.extend(other.rename);
        self.delete = self.delete.extend(other.delete);
        self.number = self.number.extend(other.number);
        self.action = self.action.extend(other.action);
        self.search = self.search.extend(other.search);
        self.filter = self.filter.extend(other.filter);
        self.relative_path_does_contain = self
            .relative_path_does_contain
            .extend(other.relative_path_does_contain);
        self.relative_path_does_not_contain = self
            .relative_path_does_not_contain
            .extend(other.relative_path_does_not_contain);
        self.sort = self.sort.extend(other.sort);
        self
    }

    pub fn get(&self, name: &str) -> Option<&Mode> {
        match name {
            "default" => Some(&self.default),
            "selection ops" => Some(&self.selection_ops),
            "selection_ops" => Some(&self.selection_ops),
            "create" => Some(&self.create),
            "create file" => Some(&self.create_file),
            "create_file" => Some(&self.create_file),
            "create directory" => Some(&self.create_directory),
            "create_directory" => Some(&self.create_directory),
            "number" => Some(&self.number),
            "go to" => Some(&self.go_to),
            "go_to" => Some(&self.go_to),
            "rename" => Some(&self.rename),
            "delete" => Some(&self.delete),
            "action" => Some(&self.action),
            "search" => Some(&self.search),
            "sort" => Some(&self.sort),
            "filter" => Some(&self.filter),
            "relative_path_does_contain" => Some(&self.relative_path_does_contain),
            "relative path does contain" => Some(&self.relative_path_does_contain),
            "relative_path_does_not_contain" => Some(&self.relative_path_does_not_contain),
            "relative path does not contain" => Some(&self.relative_path_does_not_contain),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModesConfig {
    #[serde(default)]
    pub builtin: BuiltinModesConfig,

    #[serde(default)]
    pub custom: HashMap<String, Mode>,
}

impl ModesConfig {
    pub fn get(&self, name: &str) -> Option<&Mode> {
        self.builtin.get(name).or_else(|| self.custom.get(name))
    }

    pub fn extend(mut self, other: Self) -> Self {
        self.builtin = self.builtin.extend(other.builtin);
        self.custom.extend(other.custom);
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: String,

    #[serde(default)]
    pub general: GeneralConfig,

    #[serde(default)]
    pub node_types: NodeTypesConfig,

    #[serde(default)]
    pub modes: ModesConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: default_config::version(),
            general: default_config::general(),
            node_types: default_config::node_types(),
            modes: default_config::modes(),
        }
    }
}

impl Config {
    pub fn extended(mut self) -> Self {
        let default = Self::default();
        self.general = default.general.extend(self.general);
        self.node_types = default.node_types.extend(self.node_types);
        self.modes = default.modes.extend(self.modes);
        self
    }

    fn parsed_version(&self) -> Result<(u16, u16, u16)> {
        let mut configv = self
            .version
            .strip_prefix('v')
            .unwrap_or_default()
            .split('.');

        let major = configv.next().unwrap_or_default().parse::<u16>()?;
        let minor = configv.next().unwrap_or_default().parse::<u16>()?;
        let bugfix = configv.next().unwrap_or_default().parse::<u16>()?;

        Ok((major, minor, bugfix))
    }

    pub fn is_compatible(&self) -> Result<bool> {
        let result = match self.parsed_version()? {
            (0, 5, 4) => true,
            (0, 5, 3) => true,
            (0, 5, 2) => true,
            (0, 5, 1) => true,
            (0, 5, 0) => true,
            (_, _, _) => false,
        };

        Ok(result)
    }

    pub fn upgrade_notification(&self) -> Result<Option<&str>> {
        let result = match self.parsed_version()? {
            (0, 5, 5) => None,
            (0, 5, 4) => Some("App version updated. Significant reduction in CPU usage"),
            (0, 5, 3) => Some("App version updated. Fixed exit on permission denied"),
            (0, 5, 2) => Some("App version updated. Now pwd is synced with your terminal session"),
            (0, 5, 1) => Some("App version updated. Now follow symlinks using 'gf'"),
            (_, _, _) => Some("App version updated. New: added sort and filter support and some hacks: https://github.com/sayanarijit/xplr/wiki/Hacks"),
        };

        Ok(result)
    }
}
