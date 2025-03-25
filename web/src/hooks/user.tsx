"use client";

import type { CurrentUser } from "@/model/auth";
import { type Dispatch, type SetStateAction, createContext, useContext, useState } from "react";

type CurrentUserContext = {
  currentUser?: CurrentUser;
  setUser: Dispatch<SetStateAction<CurrentUser | undefined>>;
};

const Context = createContext<CurrentUserContext | undefined>(undefined);

export function CurrentUserContext({
  children,
}: {
  children: React.ReactNode;
}) {
  const [currentUser, setUser] = useState<CurrentUser | undefined>(undefined);

  return <Context.Provider value={{ currentUser, setUser }}>{children}</Context.Provider>;
}

export function useCurrentUser(): CurrentUser | undefined {
  const currentUser = useContext(Context);
  // TODO: Maybe verify the user is still logged in?

  return currentUser?.currentUser;
}

export function setCurrentUser(user?: CurrentUser) {
  useContext(Context)?.setUser(user);
}
