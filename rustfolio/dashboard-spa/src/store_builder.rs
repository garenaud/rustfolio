use serde::{Deserialize, Serialize};
use yewdux::store::Store;

/// Type de widgets (placeholder). Plus tard on branchera sur la DB.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum WidgetKind {
    Text(String),
    ProfileBasic,          // ex: nom + titre
    ExperienceTimeline,    // expériences
    SkillsGrid,            // compétences
    ProjectsList,          // projets
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Column {
    pub id: usize,
    pub widgets: Vec<WidgetKind>,
}

impl Default for Column {
    fn default() -> Self {
        Self { id: 0, widgets: vec![] }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Row {
    pub id: usize,
    pub columns: Vec<Column>,
}

impl Default for Row {
    fn default() -> Self {
        Self { id: 0, columns: vec![Column { id: 0, widgets: vec![] }] }
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct BuilderLayout {
    pub rows: Vec<Row>,
    pub selected_row: Option<usize>,
    pub selected_column: Option<usize>, // NEW
    pub next_row_id: usize,
    pub next_col_id: usize,
}

impl BuilderLayout {
    pub fn add_row(&mut self) {
        let row_id = { self.next_row_id += 1; self.next_row_id };
        let col_id = { self.next_col_id += 1; self.next_col_id };
        self.rows.push(Row {
            id: row_id,
            columns: vec![Column { id: col_id, widgets: vec![] }],
        });
        // sélectionne la nouvelle ligne par UX
        self.selected_row = Some(row_id);
        self.selected_column = None;
    }

    pub fn select_row(&mut self, row_id: usize) {
        self.selected_row = Some(row_id);
        self.selected_column = None; // reset la colonne quand on change de ligne
    }

    pub fn select_column(&mut self, col_id: usize) {
        self.selected_column = Some(col_id);
    }

    pub fn split_selected_row(&mut self, n: usize) {
        if n == 0 { return; }
        let Some(sel) = self.selected_row else { return; };
        if let Some(row) = self.rows.iter_mut().find(|r| r.id == sel) {
            // récup widgets existants
            let mut old_widgets = Vec::new();
            for col in &mut row.columns {
                old_widgets.append(&mut col.widgets);
            }

            // recréer n colonnes
            row.columns = (0..n).map(|_| {
                self.next_col_id += 1;
                Column { id: self.next_col_id, widgets: vec![] }
            }).collect();

            // répartir en round-robin
            for (i, w) in old_widgets.into_iter().enumerate() {
                let idx = i % n;
                row.columns[idx].widgets.push(w);
            }

            // si la colonne sélectionnée n'existe plus -> désélection
            if let Some(sel_col) = self.selected_column {
                if !row.columns.iter().any(|c| c.id == sel_col) {
                    self.selected_column = None;
                }
            }
        }
    }

    pub fn add_widget_to_selected_column(&mut self, w: WidgetKind) {
        let Some(col_id) = self.selected_column else { return; };
        for row in &mut self.rows {
            if let Some(col) = row.columns.iter_mut().find(|c| c.id == col_id) {
                col.widgets.push(w);
                break;
            }
        }
    }
}

impl Store for BuilderLayout {
    fn new(_ctx: &yewdux::Context) -> Self {
        Self::default()
    }

    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}
