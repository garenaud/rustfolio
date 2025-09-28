use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

/// Un "widget" minimal pour la démo.
/// Plus tard, tu pourras typer les widgets (Texte, Expérience, Skills, etc.)
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct Widget {
    pub kind: String,
    pub payload: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct Column {
    pub id: usize,
    pub widgets: Vec<Widget>,
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
    }

    pub fn select_row(&mut self, row_id: usize) {
        self.selected_row = Some(row_id);
    }

    pub fn split_selected_row(&mut self, n: usize) {
        if n == 0 { return; }
        let Some(sel) = self.selected_row else { return; };
        if let Some(row) = self.rows.iter_mut().find(|r| r.id == sel) {
            // Récupérer les widgets existants pour les redistribuer
            let mut old_widgets = Vec::new();
            for col in &mut row.columns {
                old_widgets.append(&mut col.widgets);
            }

            // Recréer n colonnes
            row.columns = (0..n).map(|_| {
                self.next_col_id += 1;
                Column { id: self.next_col_id, widgets: vec![] }
            }).collect();

            // Redistribution simple en round-robin
            for (i, w) in old_widgets.into_iter().enumerate() {
                let idx = i % n;
                row.columns[idx].widgets.push(w);
            }
        }
    }
}

// Rendre le store observable par Yewdux
impl yewdux::store::Store for BuilderLayout {
    fn new(_ctx: &yewdux::Context) -> Self {
        Self::default()
    }

    // Notifie le UI seulement si l'état change réellement
    fn should_notify(&self, old: &Self) -> bool {
        self != old
    }
}
