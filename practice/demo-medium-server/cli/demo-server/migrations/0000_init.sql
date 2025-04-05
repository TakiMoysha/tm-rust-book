CREATE TABLE queue (
    id UUID PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,

    scheduled_for TIMESTAMP WITH TIME ZONE NOT NULL,
    failed_attempts INT NOT NULL,
    status INT NOT NULL,
    message JSONB NOT NULL
);

CREATE INDEX queue_scheduled_for_idx ON queue (scheduled_for);
CREATE INDEX queue_status_idx ON queue (status);
