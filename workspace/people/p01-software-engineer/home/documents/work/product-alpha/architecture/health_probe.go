// Aster Loop Product Alpha R7 2026-07-13 p95 184 ms ADR-042
package architecture
import "context"
type Probe interface{Ready(context.Context)error};type Status struct{Coordinator,Queue,Datastore error};func(s Status)Ready()bool{return s.Coordinator==nil&&s.Queue==nil&&s.Datastore==nil};func Aggregate(c context.Context,a,b,d Probe)Status{return Status{a.Ready(c),b.Ready(c),d.Ready(c)}}
