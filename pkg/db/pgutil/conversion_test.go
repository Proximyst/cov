package pgutil_test

import (
	"testing"
	"time"

	"github.com/google/uuid"
	"github.com/jackc/pgx/v5/pgtype"
	"github.com/proximyst/cov/pkg/db/pgutil"
	"github.com/stretchr/testify/assert"
)

func TestUUIDs(t *testing.T) {
	t.Parallel()

	t.Run("from valid google UUID", func(t *testing.T) {
		t.Parallel()

		id := uuid.New()
		pgID := pgutil.FromGoogleUUID(id)
		assert.True(t, pgID.Valid, "pgtype.UUID should be valid")
		assert.Equal(t, [16]byte(id), pgID.Bytes, "pgtype.UUID bytes should match the original UUID")
	})

	t.Run("from nil google UUID", func(t *testing.T) {
		t.Parallel()

		id := uuid.Nil
		pgID := pgutil.FromGoogleUUID(id)
		assert.False(t, pgID.Valid, "pgtype.UUID should not be valid")
		assert.Equal(t, [16]byte(uuid.Nil), pgID.Bytes, "pgtype.UUID bytes should be nil")
	})

	t.Run("to valid google UUID", func(t *testing.T) {
		t.Parallel()

		pgID := pgtype.UUID{
			Bytes: uuid.MustParse("0196a40d-838c-74c7-9c15-8e481098dae5"),
			Valid: true,
		}
		id := pgutil.ToGoogleUUID(pgID)
		assert.Equal(t, uuid.MustParse("0196a40d-838c-74c7-9c15-8e481098dae5"), id, "pgtype.UUID should convert back to the original UUID")
	})

	t.Run("to nil google UUID", func(t *testing.T) {
		t.Parallel()

		pgID := pgtype.UUID{
			Bytes: uuid.Nil,
			Valid: false,
		}
		id := pgutil.ToGoogleUUID(pgID)
		assert.Equal(t, uuid.Nil, id, "pgtype.UUID should convert to nil UUID")
	})
}

func TestTimestamps(t *testing.T) {
	t.Parallel()

	t.Run("from valid time", func(t *testing.T) {
		t.Parallel()

		timestamp := time.Date(2025, 1, 2, 3, 4, 5, 6, time.UTC)
		pgTimestamp := pgutil.FromTime(timestamp)
		assert.True(t, pgTimestamp.Valid, "pgtype.Timestamptz should be valid")
		assert.Equal(t, timestamp, pgTimestamp.Time, "pgtype.Timestamptz time should match the original time")
	})

	t.Run("from zero time", func(t *testing.T) {
		t.Parallel()

		timestamp := time.Time{}
		pgTimestamp := pgutil.FromTime(timestamp)
		assert.False(t, pgTimestamp.Valid, "pgtype.Timestamptz should not be valid")
		assert.Equal(t, time.Time{}, pgTimestamp.Time, "pgtype.Timestamptz time should be zero")
	})

	t.Run("to valid time", func(t *testing.T) {
		t.Parallel()

		pgTimestamp := pgtype.Timestamptz{
			Time:  time.Date(2025, 1, 2, 3, 4, 5, 6, time.UTC),
			Valid: true,
		}
		timestamp := pgutil.ToTime(pgTimestamp)
		assert.Equal(t, time.Date(2025, 1, 2, 3, 4, 5, 6, time.UTC), timestamp, "pgtype.Timestamptz should convert back to the original time")
	})

	t.Run("to zero time", func(t *testing.T) {
		t.Parallel()

		pgTimestamp := pgtype.Timestamptz{
			Time:  time.Time{},
			Valid: false,
		}
		timestamp := pgutil.ToTime(pgTimestamp)
		assert.Equal(t, time.Time{}, timestamp, "pgtype.Timestamptz should convert to zero time")
	})
}
