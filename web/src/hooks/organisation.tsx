"use client";

import type { Organisation } from "@/model/organisation";
import { type Dispatch, type SetStateAction, createContext, useContext, useState } from "react";

type OrganisationContextType = {
  currentOrg?: Organisation;
  setOrg: Dispatch<SetStateAction<Organisation | undefined>>;
};

const Context = createContext<OrganisationContextType | undefined>(undefined);

export function OrgContext({ children }: { children: React.ReactNode }) {
  const [currentOrg, setOrg] = useState<Organisation | undefined>(undefined);

  return <Context.Provider value={{ currentOrg, setOrg }}>{children}</Context.Provider>;
}

export function useOrgState(): [Organisation | undefined, Dispatch<SetStateAction<Organisation | undefined>>] {
  const context = useContext(Context);
  return [context?.currentOrg, context?.setOrg ?? (() => {})];
}

export function useOrg(): Organisation | undefined {
  return useContext(Context)?.currentOrg;
}

export function setOrg(org?: Organisation) {
  useContext(Context)?.setOrg(org);
}
