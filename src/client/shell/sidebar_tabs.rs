//! Agent-less tabs shown as a group under the agent rows.
//!
//! The endpoint projects every tab and every pane into the snapshot, and projects
//! an agent record only for panes whose terminal is an agent terminal. A tab with
//! no agent record against it therefore has no agent pane, which is exactly the
//! set this module surfaces.

use std::collections::HashMap;

use ratatui::{buffer::Buffer, layout::Rect};

use super::*;

pub(super) struct TabRow {
    pub(super) tab_id: String,
    pub(super) status: crate::api::schema::AgentStatus,
    pub(super) focused: bool,
    pub(super) rows: Vec<Vec<crate::ui::ResolvedToken>>,
}

/// Tabs that host no agent, in snapshot order, across every workspace.
pub(super) fn agentless_tabs(snapshot: &ClientShellSnapshot) -> Vec<&ClientShellTab> {
    snapshot
        .tabs
        .iter()
        .filter(|tab| {
            !snapshot
                .agents
                .iter()
                .any(|agent| agent.tab_id == tab.tab_id)
        })
        .collect()
}

pub(super) fn tab_rows(
    snapshot: &ClientShellSnapshot,
    config: &ClientShellConfig,
    machine: Option<&str>,
) -> Vec<TabRow> {
    // Non-agent panes carry no terminal title or metadata tokens on the wire, so
    // those tokens resolve to nothing for these rows.
    let empty_tokens = HashMap::new();
    agentless_tabs(snapshot)
        .into_iter()
        .filter_map(|tab| {
            let workspace = snapshot
                .workspaces
                .iter()
                .find(|workspace| workspace.workspace_id == tab.workspace_id)?;
            let panes = snapshot
                .panes
                .iter()
                .filter(|pane| pane.tab_id == tab.tab_id)
                .collect::<Vec<_>>();
            let pane = panes
                .iter()
                .find(|pane| pane.focused)
                .or_else(|| panes.first());
            let rows = crate::ui::sidebar_agent_rows(
                &config.agents,
                crate::ui::AgentTokenContext {
                    machine,
                    workspace: &workspace.label,
                    tab: Some(tab.label.as_str()),
                    pane: pane.and_then(|pane| pane.label.as_deref()),
                    agent_label: None,
                    terminal_title: None,
                    terminal_title_stripped: None,
                    canonical_agent: None,
                    tokens: &empty_tokens,
                },
                None,
            );
            Some(TabRow {
                tab_id: tab.tab_id.clone(),
                status: tab.agent_status,
                focused: tab.focused,
                rows,
            })
        })
        .collect()
}

pub(super) fn render_tab_row(
    buffer: &mut Buffer,
    rect: Rect,
    row: &TabRow,
    config: &ClientShellConfig,
) {
    super::agent_sidebar::render_token_rows(
        buffer,
        rect,
        row.status,
        row.focused,
        &row.rows,
        config,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::{ClientShellAgent, ClientShellTab};

    fn tab(tab_id: &str, workspace_id: &str, label: &str) -> ClientShellTab {
        ClientShellTab {
            tab_id: tab_id.to_owned(),
            workspace_id: workspace_id.to_owned(),
            number: 1,
            label: label.to_owned(),
            custom_label: false,
            zoomed: false,
            focused: false,
            agent_status: crate::api::schema::AgentStatus::Idle,
        }
    }

    fn agent(pane_id: &str, tab_id: &str, workspace_id: &str) -> ClientShellAgent {
        ClientShellAgent {
            pane_id: pane_id.to_owned(),
            workspace_id: workspace_id.to_owned(),
            tab_id: tab_id.to_owned(),
            name: None,
            display_agent: None,
            agent: None,
            title: None,
            terminal_title: None,
            terminal_title_stripped: None,
            agent_status: crate::api::schema::AgentStatus::Idle,
            state_change_seq: 0,
            state_labels: Vec::new(),
            tokens: Vec::new(),
            focused: false,
        }
    }

    #[test]
    fn agentless_tabs_exclude_tabs_hosting_an_agent() {
        let mut snapshot = super::super::tests::snapshot();
        snapshot.tabs = vec![
            tab("w1:t1", "w1", "agents"),
            tab("w1:t2", "w1", "scratch"),
            tab("w2:t1", "w2", "notes"),
        ];
        snapshot.agents = vec![agent("w1:p1", "w1:t1", "w1")];

        let labels = agentless_tabs(&snapshot)
            .into_iter()
            .map(|tab| tab.label.as_str())
            .collect::<Vec<_>>();

        assert_eq!(labels, vec!["scratch", "notes"]);
    }

    #[test]
    fn agentless_tabs_are_empty_when_every_tab_hosts_an_agent() {
        let mut snapshot = super::super::tests::snapshot();
        snapshot.tabs = vec![tab("w1:t1", "w1", "agents")];
        snapshot.agents = vec![agent("w1:p1", "w1:t1", "w1")];

        assert!(agentless_tabs(&snapshot).is_empty());
    }
}
