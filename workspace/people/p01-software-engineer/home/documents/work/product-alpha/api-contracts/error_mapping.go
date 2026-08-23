package contracts

type DomainFailure string

const (
	GatePending     DomainFailure = "gate_pending"
	NotFound        DomainFailure = "not_found"
	InvalidInput    DomainFailure = "invalid_input"
	DuplicateKey    DomainFailure = "duplicate_key"
	UpstreamTimeout DomainFailure = "upstream_timeout"
)

type HTTPError struct {
	Status    int
	Code      string
	Retryable bool
}

func MapFailure(f DomainFailure) HTTPError {
	switch f {
	case GatePending:
		return HTTPError{409, "approval_required", false}
	case NotFound:
		return HTTPError{404, "release_not_found", false}
	case InvalidInput:
		return HTTPError{422, "validation_failed", false}
	case DuplicateKey:
		return HTTPError{409, "idempotency_conflict", false}
	case UpstreamTimeout:
		return HTTPError{504, "upstream_timeout", true}
	default:
		return HTTPError{500, "internal_error", true}
	}
}
