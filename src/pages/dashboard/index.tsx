import { PageLayout } from "@/layouts";

const Dashboard = () => {
  return (
    <PageLayout
      title="Dashboard"
      description="Crystall - your private AI assistant for meetings, interviews, and conversations."
    >
      <div className="flex flex-col gap-3 p-4 select-none">
        <div className="rounded-lg border p-4">
          <h3 className="font-semibold mb-1">Welcome</h3>
          <p className="text-muted-foreground text-sm">
            Configure your AI and speech providers in Settings to get started.
            Crystall works fully offline with your own API keys - no account or
            license required.
          </p>
        </div>
      </div>
    </PageLayout>
  );
};

export default Dashboard;
