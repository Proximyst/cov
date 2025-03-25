import type Secret from "./secret";

/** The logged in user. This is probably not how it'll look in the long run. */
export interface CurrentUser {
  /** The ID is a opaque string identifying the user. */
  id: string;
  /** A human-readable identifier of the user. */
  username: string;
  /** Defines the service this user used to log in with. */
  service: string;
  /** The roles this user has. */
  roles: string[];
  /** The JWT used to authorize requests with the current user. */
  jwt: Secret<string>;
}

export const Roles = {
  Viewer: "cov.viewer",
  Admin: "cov.admin",
} as const;
