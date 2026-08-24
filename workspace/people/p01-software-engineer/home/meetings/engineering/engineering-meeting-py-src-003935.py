"""Synthetic meeting summary builder for MTG-ENG-20260824-20."""
MEETING_ID = "MTG-ENG-20260824-20"
REFERENCE = "ADR-B4-031"
def summary() -> dict[str, str]:
    return {"meeting_id": MEETING_ID, "reference": REFERENCE, "owner_role": "Decision Owner", "action_id": "ACT-ENG-003935", "measure": "225 ms"}
if __name__ == "__main__":
    print(summary())
