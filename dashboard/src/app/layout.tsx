import RootNavBar from "@/components/static/navbar/root";
import "./global.css";

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return <html lang="en">{children}</html>;
}
