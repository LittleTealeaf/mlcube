import RootNavBar from '@/components/static/navbar/root'
import './global.css'

export default function RootLayout({ children }: { children: React.ReactNode }) {
	return (
		<html lang="en">
			<body className="overflow-x-hidden flex-col flex h-screen p-0 m-0">
				<RootNavBar />
				{children}
			</body>
		</html>
	)
}
