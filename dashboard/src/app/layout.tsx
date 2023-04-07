// "use client"
import './global.css'
import NavBar from '@/components/navbar';

export default function RootLayout({ children }: { children: React.ReactNode }) {


	return (
		<html lang="en">
			<body>
				<NavBar />
				{children}
			</body>
		</html>
	)
}
