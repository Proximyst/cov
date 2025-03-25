"use client";

import { Header } from "@/components/header";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { VerticalSeparator } from "@/components/vertical-separator";
import { OrgContext } from "@/hooks/organisation";
import { CurrentUserContext } from "@/hooks/user";

function RepositoryCard({ name }: { name?: string }) {
  return (
    <Card className="w-64 not-md:w-48 h-64 not-md:h-48">
      <CardHeader>
        <CardTitle>{name ?? "repo here"}</CardTitle>
      </CardHeader>
    </Card>
  );
}

function RepositoryListOptions() {
  return <>Repositories | Settings | Search (TODO)</>;
}

function RepositoryList() {
  return (
    <div className="grid grid-cols-2 not-md:grid-cols-1 p-8 gap-8 align-top">
      <RepositoryCard name="repo 1" />
      <RepositoryCard name="repo 2" />
      <RepositoryCard name="very-long-repository-name-here-thats-wild-isnt-it-wow-i-should-recite-a-poem" />
      <RepositoryCard name="repo 2" />
      <RepositoryCard name="repo 2" />
      <RepositoryCard name="repo 2" />
      <RepositoryCard name="repo 2" />
    </div>
  );
}

function RepositoryDetailsOptions() {
  return <>Coverage | Commits | PRs | Settings (TODO)</>;
}

function RepositoryDetails() {
  return (
    <div className="flex flex-col p-8 mr-8">
      <div className="grow">this will be the right side</div>
    </div>
  );
}

export default function Overview() {
  return (
    <CurrentUserContext>
      <OrgContext>
        <div className="flex flex-col gap-4 min-h-screen">
          <Header />

          <div className="grow flex flex-row gap-0 px-4 pb-4 min-h-full">
            <div className="shrink flex flex-col">
              <RepositoryListOptions />
              <Separator orientation="horizontal" />
              <RepositoryList />
            </div>
            <VerticalSeparator className="shrink h-auto" />
            <div className="grow flex flex-col">
              <RepositoryDetailsOptions />
              <Separator orientation="horizontal" />
              <RepositoryDetails />
            </div>
          </div>
        </div>
      </OrgContext>
    </CurrentUserContext>
  );
}
