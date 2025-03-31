"use client";

import { Header } from "@/components/header";
import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle } from "@/components/ui/card";
import { Separator } from "@/components/ui/separator";
import { useState } from "react";

function Pages() {
  const [currentPage, setPage] = useState(0);
  const btn = (i: number, txt: string) => (
    <Button
      variant={currentPage == i ? "secondary" : "ghost"}
      className="text-base"
      onClick={() => setPage(i)}
    >
      {txt}
    </Button>
  );

  return (
    <div className="flex flex-row gap-4 min-w-full">
      {btn(0, "Repositories")}
      {btn(1, "Settings")}
    </div>
  );
}

function RepositoryCard({ name }: { name?: string }) {
  return (
    <Card
      className={
        "grow w-96 not-md:w-64 max-w-128 h-64 not-md:h-48" +
        " hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50" +
        " transition duration-150 ease-in-out"
      }
    >
      <CardHeader>
        <CardTitle>{name ?? "repo here"}</CardTitle>
      </CardHeader>
    </Card>
  );
}

export default function Repositories() {
  return (
    <div className="flex flex-col gap-4 min-h-screen">
      <Header />

      <div className="grow flex flex-col gap-2 min-h-full px-6">
        <Pages />
        <Separator className="shrink w-auto" />
        <div className="grow flex flex-row flex-wrap gap-4 min-w-full content-center">
          {new Array(50).fill(0).map((_, i) => (
            <RepositoryCard key={i} />
          ))}
        </div>
      </div>
    </div>
  );
}
