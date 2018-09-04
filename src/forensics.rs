use ::colored::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct FilePos(pub usize);

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TraceEvent {
    MatchStart {
        rule_name: &'static str,
        start_pos: FilePos,
    },
    MatchEnd {
        rule_name: &'static str,
        start_pos: FilePos,
        end_pos: Option<FilePos>,
    },
    CachedRule {
        rule_name: &'static str,
        start_pos: FilePos,
        end_pos: Option<FilePos>,
    },
}

pub struct EventData {
    event: TraceEvent,
    depth: usize,
    pair_index: Option<usize>,
}

pub struct TraceResult {
    events: Vec<TraceEvent>,
}

pub fn investigate(text: &str, trace: &[TraceEvent]) {
    let mut out = Vec::new();
    let mut depth = 0;

    let mut current_stack: Vec<usize> = Vec::new();

    for (idx, event) in trace.iter().enumerate() {
        match event {
            TraceEvent::MatchStart { .. } => {
                out.push(EventData {
                    event: *event,
                    depth: depth,
                    pair_index: None,
                });
                current_stack.push(idx);
                depth += 1;
            },
            TraceEvent::MatchEnd { .. } => {
                depth -= 1;
                out.push(EventData {
                    event: *event,
                    depth: depth,
                    pair_index: Some(current_stack.pop().unwrap()),
                });
            },
            _ => unimplemented!(),
        }
    }
    assert!(depth == 0);

    let mut map: Vec<(usize, usize)> = Vec::new();
    for (idx, entry) in out.iter().enumerate() {
        if let Some(other_idx) = entry.pair_index {
            map.push((other_idx, idx));
        }
    }
    for (idx, target) in map {
        let mut other = &mut out[idx];
        assert!(other.pair_index.is_none());
        other.pair_index = Some(target);
    }

    let lines = text.split("\n");

    let mut idx = 0;
    loop {
        let item = &out[idx];

        if let TraceEvent::MatchStart { .. } = item.event {
            let other_idx = item.pair_index.unwrap();
            if let TraceEvent::MatchEnd { end_pos: Some(_), .. } = out[other_idx].event {
                idx = other_idx;
            }
            if other_idx == idx + 1 {
                idx += 1;
            }
        }

        let item = &out[idx];

        let indent = "  ".repeat(item.depth);
        match item.event {
            TraceEvent::MatchStart { rule_name, .. } => println!("{}{}", indent, rule_name.blue()),
            TraceEvent::MatchEnd { rule_name, end_pos: Some(_), .. } => 
                println!("{}{}", indent, rule_name.green()),
            TraceEvent::MatchEnd { rule_name, end_pos: None, .. } => 
                println!("{}{}", indent, rule_name.red()),
            _ => unimplemented!(),
        }

        idx += 1;
        if (idx >= out.len()) {
            break;
        }
    }

}

use ::cursive::Cursive;
use ::cursive::view::Identifiable;
use ::cursive::views::Panel;

pub fn start_interactive(text: &str, trace: &[TraceEvent]) {
    let mut siv = Cursive::new();


    siv.run();
}
