# Aster Loop Product Alpha R7 2026-07-13 p95 184 ms ADR-042
import json,sys
def compatible(old,new):
 if old.get("major")!=new.get("major"):return False,"major version changed"
 m=sorted(set(old.get("required",[]))-set(new.get("required",[])));return not m,("compatible" if not m else "missing required: "+", ".join(m))
if __name__=="__main__":
 ok,msg=compatible(json.load(open(sys.argv[1])),json.load(open(sys.argv[2])));print(msg);raise SystemExit(0 if ok else 2)
