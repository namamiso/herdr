use crate::api::schema::{
    AgentRenameParams, EmptyParams, LayoutSetSplitRatioParams, Method, PaneFocusDirectionParams,
    PaneRenameParams, PaneResizeParams, PaneSplitParams, PaneSwapParams, PaneTarget,
    PaneZoomParams, TabCreateParams, TabMoveParams, TabRenameParams, TabTarget,
    WorkspaceCreateParams, WorkspaceMoveParams, WorkspaceRenameParams, WorkspaceTarget,
    WorktreeCreateParams, WorktreeOpenParams, WorktreeRemoveParams,
};

use super::App;

impl App {
    pub(crate) fn dispatch_runtime_mutation(&mut self, id: &'static str, method: Method) -> String {
        self.dispatch_api_request(id, method)
    }

    pub(crate) fn dispatch_deferred_runtime_mutation(
        &mut self,
        id: &'static str,
        method: Method,
    ) -> Option<String> {
        self.dispatch_deferred_api_request(id, method)
    }

    pub(crate) fn runtime_workspace_focus(
        &mut self,
        id: &'static str,
        workspace_id: String,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::WorkspaceFocus(WorkspaceTarget { workspace_id }))
    }

    pub(crate) fn runtime_workspace_create(
        &mut self,
        id: &'static str,
        params: WorkspaceCreateParams,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::WorkspaceCreate(params))
    }

    pub(crate) fn runtime_workspace_rename(
        &mut self,
        id: &'static str,
        params: WorkspaceRenameParams,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::WorkspaceRename(params))
    }

    pub(crate) fn runtime_workspace_move(
        &mut self,
        id: &'static str,
        params: WorkspaceMoveParams,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::WorkspaceMove(params))
    }

    pub(crate) fn runtime_workspace_close(
        &mut self,
        id: &'static str,
        workspace_id: String,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::WorkspaceClose(WorkspaceTarget { workspace_id }))
    }

    pub(crate) fn runtime_tab_create(
        &mut self,
        id: &'static str,
        params: TabCreateParams,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::TabCreate(params))
    }

    pub(crate) fn runtime_tab_focus(&mut self, id: &'static str, tab_id: String) -> String {
        self.dispatch_runtime_mutation(id, Method::TabFocus(TabTarget { tab_id }))
    }

    pub(crate) fn runtime_tab_rename(
        &mut self,
        id: &'static str,
        params: TabRenameParams,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::TabRename(params))
    }

    pub(crate) fn runtime_tab_move(&mut self, id: &'static str, params: TabMoveParams) -> String {
        self.dispatch_runtime_mutation(id, Method::TabMove(params))
    }

    pub(crate) fn runtime_tab_close(&mut self, id: &'static str, tab_id: String) -> String {
        self.dispatch_runtime_mutation(id, Method::TabClose(TabTarget { tab_id }))
    }

    pub(crate) fn runtime_server_reload_config(&mut self, id: &'static str) -> String {
        self.dispatch_runtime_mutation(id, Method::ServerReloadConfig(EmptyParams::default()))
    }

    pub(crate) fn runtime_pane_focus(&mut self, id: &'static str, pane_id: String) -> String {
        self.dispatch_runtime_mutation(id, Method::PaneFocus(PaneTarget { pane_id }))
    }

    pub(crate) fn runtime_pane_close(&mut self, id: &'static str, pane_id: String) -> String {
        self.dispatch_runtime_mutation(id, Method::PaneClose(PaneTarget { pane_id }))
    }

    pub(crate) fn runtime_pane_rename(
        &mut self,
        id: &'static str,
        params: PaneRenameParams,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::PaneRename(params))
    }

    pub(crate) fn runtime_agent_rename(
        &mut self,
        id: &'static str,
        params: AgentRenameParams,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::AgentRename(params))
    }

    /// Names a pane from an interactive rename. When the pane hosts an agent the
    /// name is applied as the agent name (which also keeps the pane label in
    /// sync and auto-disambiguates duplicates); otherwise it is a plain pane
    /// label. `name = None` clears whichever applies.
    pub(crate) fn rename_pane_or_agent(
        &mut self,
        ws_idx: usize,
        pane_id: crate::layout::PaneId,
        name: Option<String>,
    ) {
        let Some(pane_id_public) = self.public_pane_id(ws_idx, pane_id) else {
            return;
        };
        if self.pane_hosts_agent(ws_idx, pane_id) {
            self.runtime_agent_rename(
                "tui.agent.rename",
                AgentRenameParams {
                    target: pane_id_public,
                    name,
                },
            );
        } else {
            self.runtime_pane_rename(
                "tui.pane.rename",
                PaneRenameParams {
                    pane_id: pane_id_public,
                    label: name,
                },
            );
        }
    }

    fn pane_hosts_agent(&self, ws_idx: usize, pane_id: crate::layout::PaneId) -> bool {
        self.state
            .workspaces
            .get(ws_idx)
            .and_then(|ws| ws.pane_state(pane_id))
            .and_then(|pane| self.state.terminals.get(&pane.attached_terminal_id))
            .is_some_and(|terminal| terminal.is_agent_terminal())
    }

    pub(crate) fn runtime_pane_focus_direction(
        &mut self,
        id: &'static str,
        params: PaneFocusDirectionParams,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::PaneFocusDirection(params))
    }

    pub(crate) fn runtime_pane_resize(
        &mut self,
        id: &'static str,
        params: PaneResizeParams,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::PaneResize(params))
    }

    pub(crate) fn runtime_pane_swap(&mut self, id: &'static str, params: PaneSwapParams) -> String {
        self.dispatch_runtime_mutation(id, Method::PaneSwap(params))
    }

    pub(crate) fn runtime_pane_split(
        &mut self,
        id: &'static str,
        params: PaneSplitParams,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::PaneSplit(params))
    }

    pub(crate) fn runtime_pane_zoom(&mut self, id: &'static str, params: PaneZoomParams) -> String {
        self.dispatch_runtime_mutation(id, Method::PaneZoom(params))
    }

    pub(crate) fn runtime_layout_set_split_ratio(
        &mut self,
        id: &'static str,
        params: LayoutSetSplitRatioParams,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::LayoutSetSplitRatio(params))
    }

    pub(crate) fn runtime_worktree_create_deferred(
        &mut self,
        id: &'static str,
        params: WorktreeCreateParams,
    ) -> Option<String> {
        self.dispatch_deferred_runtime_mutation(id, Method::WorktreeCreate(params))
    }

    pub(crate) fn runtime_worktree_open(
        &mut self,
        id: &'static str,
        params: WorktreeOpenParams,
    ) -> String {
        self.dispatch_runtime_mutation(id, Method::WorktreeOpen(params))
    }

    pub(crate) fn runtime_worktree_remove_deferred(
        &mut self,
        id: &'static str,
        params: WorktreeRemoveParams,
    ) -> Option<String> {
        self.dispatch_deferred_runtime_mutation(id, Method::WorktreeRemove(params))
    }
}
