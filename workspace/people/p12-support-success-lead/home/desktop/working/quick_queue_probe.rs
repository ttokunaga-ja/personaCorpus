// Offline probe for the canonical 2026-08-04 HelioDesk frozen snapshot.
#[derive(Clone, Copy)]
struct Ticket { id: &'static str, priority: &'static str, customer: &'static str, sla: &'static str }
fn main() {
    let q = [
      Ticket{id:"HD-260804-2718",priority:"P1",customer:"Customer Alpha",sla:"breached"}, Ticket{id:"HD-260804-1934",priority:"P2",customer:"Customer Beta",sla:"breached"}, Ticket{id:"HD-260804-2841",priority:"P2",customer:"Customer Alpha",sla:"at-risk"}, Ticket{id:"HD-260804-2849",priority:"P2",customer:"Other",sla:"within"}, Ticket{id:"HD-260804-2857",priority:"P2",customer:"Customer Alpha",sla:"within"}, Ticket{id:"HD-260804-2861",priority:"P2",customer:"Customer Beta",sla:"at-risk"}, Ticket{id:"HD-260804-2726",priority:"P3",customer:"Customer Alpha",sla:"at-risk"}, Ticket{id:"HD-260804-2732",priority:"P3",customer:"Customer Alpha",sla:"within"}, Ticket{id:"HD-260804-2740",priority:"P3",customer:"Customer Beta",sla:"within"}, Ticket{id:"HD-260804-2751",priority:"P3",customer:"Other",sla:"within"}, Ticket{id:"HD-260804-2763",priority:"P3",customer:"Customer Alpha",sla:"at-risk"}, Ticket{id:"HD-260804-2774",priority:"P3",customer:"Customer Beta",sla:"within"}, Ticket{id:"HD-260804-2789",priority:"P3",customer:"Customer Alpha",sla:"within"}, Ticket{id:"HD-260804-2803",priority:"P3",customer:"Customer Beta",sla:"within"}, Ticket{id:"HD-260804-2811",priority:"P4",customer:"Customer Alpha",sla:"within"}, Ticket{id:"HD-260804-2824",priority:"P4",customer:"Customer Alpha",sla:"within"}, Ticket{id:"HD-260804-2836",priority:"P4",customer:"Other",sla:"within"},
    ];
    let count = |field: &str, value: &str| match field { "priority" => q.iter().filter(|t|t.priority==value).count(), "customer" => q.iter().filter(|t|t.customer==value).count(), _ => q.iter().filter(|t|t.sla==value).count() };
    println!("unresolved={}; P1/P2/P3/P4={}/{}/{}/{}; SLA within/at-risk/breached={}/{}/{}; Alpha/Beta/other={}/{}/{}", q.len(), count("priority","P1"),count("priority","P2"),count("priority","P3"),count("priority","P4"),count("sla","within"),count("sla","at-risk"),count("sla","breached"),count("customer","Customer Alpha"),count("customer","Customer Beta"),count("customer","Other"));
    assert_eq!(q.len(),17); assert_eq!(q[0].id,"HD-260804-2718"); assert_eq!(q[0].sla,"breached");
    assert_eq!((count("priority","P1"),count("priority","P2"),count("priority","P3"),count("priority","P4")),(1,5,8,3));
    assert_eq!((count("sla","within"),count("sla","at-risk"),count("sla","breached")),(11,4,2));
    assert_eq!((count("customer","Customer Alpha"),count("customer","Customer Beta"),count("customer","Other")),(9,5,3));
}
