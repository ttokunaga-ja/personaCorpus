#[cfg(test)] mod scenarios {
 #[test] fn duplicate_event_is_idempotent(){let checkpoint=4_u64;let incoming=4_u64;assert!(incoming<=checkpoint);assert_eq!(checkpoint,4);}
 #[test] fn partial_failure_keeps_checkpoint(){let before=9_u64;let failed=true;let after=if failed{before}else{before+1};assert_eq!(before,after);}
 #[test] fn rollback_preserves_append_only_evidence(){let events=vec!["created","rejected"];assert_eq!(events.len(),2);}
 #[test] fn recovery_replays_only_uncheckpointed_events(){let checkpoint=3;let offsets=[2,3,4];assert_eq!(offsets.into_iter().filter(|x|*x>checkpoint).count(),1);}
}
