import json, sys
from pathlib import Path
p=json.loads(Path(sys.argv[1] if len(sys.argv)>1 else "protocol-parameter-contract.json").read_text())
required={"organization":"Orchid Cellworks","study":"Study Alpha","date":"2026-07-13","cohort":"cohort A-24","assay_target":"assay 6.8 ng/mL"}
assert all(p.get(k)==v for k,v in required.items()) and p.get("synthetic") is True
print("protocol parameters valid")
