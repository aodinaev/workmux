/**
 * Workmux status tracking extension for oh-my-pi.
 *
 * Reports agent status to workmux for tmux window status display.
 * See: https://workmux.raine.dev/guide/status-tracking
 */

import type { ExtensionAPI } from "@oh-my-pi/pi-coding-agent";

export default function (pi: ExtensionAPI) {
  function setStatus(status: string) {
    pi.exec("workmux", ["set-window-status", status]).catch(() => {});
  }

  pi.on("agent_start", async () => {
    setStatus("working");
  });

  pi.on("message_update", async () => {
    setStatus("working");
  });

  pi.on("message_end", async (event) => {
    if ("role" in event.message && event.message.role === "assistant") {
      setStatus("waiting");
    }
  });

  pi.on("tool_call", async (event) => {
    if (event.toolName === "ask") {
      setStatus("waiting");
    } else {
      setStatus("working");
    }
  });

  pi.on("tool_execution_start", async () => {
    setStatus("working");
  });

  pi.on("agent_end", async () => {
    setStatus("done");
  });
}
