"""Render a parameterized, non-networked MTR-09 query preview."""
from datetime import date


def build_query(start: date, end: date) -> str:
    if start > end:
        raise ValueError("start must not follow end")
    return ("SELECT business_date, SUM(merchandise_jpy-recognized_refund_jpy) "
            "AS gmv_jpy FROM analytics_synthetic.fct_order_line "
            f"WHERE completed_at::date BETWEEN DATE '{start}' AND DATE '{end}' "
            "AND order_status='completed' GROUP BY 1 ORDER BY 1;")


if __name__ == "__main__":
    print(build_query(date(2026, 7, 7), date(2026, 7, 13)))
