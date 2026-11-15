"""Synthetic Study Alpha cohort A-24 analytical pipeline."""
import csv, statistics
def summarize(path):
    with open(path, newline='') as handle:
        rows=list(csv.DictReader(handle))
    vals=[float(r['assay_ng_mL']) for r in rows]
    return {'n':len(vals),'median_ng_mL':statistics.median(vals),'study':'Study Alpha'}
if __name__=='__main__':
    print(summarize('alpha-a24-clean-analysis-table.csv'))
