export interface Organisation {
  /** The ID is a opaque string identifying the organisation. */
  id: string;
  /** A human-readable identifier of the organisation. */
  name: string;
  /** Defines the service this organisation exists on. */
  service: string;
}
