#!/usr/bin/env python3
"""Offline dependency health summary for Product Alpha Q3."""
import csv, pathlib, sys
from datetime import date
ROOT=pathlib.Path(__file__).resolve().parent
CSV=ROOT/'dependency-escalations.csv'
SNAPSHOT=date(2026,7,13)
def main():
    with CSV.open(newline='', encoding='utf-8') as f: rows=list(csv.DictReader(f))
    by_status={}
    critical=0
    for row in rows:
        by_status[row['status']]=by_status.get(row['status'],0)+1
        critical += row['severity']=='Critical'
    print('Product Alpha Q3 dependency health')
    print(f'escalations={len(rows)} critical={critical}')
    print('status=' + ', '.join(f'{k}:{v}' for k,v in sorted(by_status.items())))
    return 0
if __name__=='__main__': sys.exit(main())
