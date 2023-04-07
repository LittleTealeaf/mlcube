// "use client"
import NavBar from '@/components/client/navbar'
import './global.css'

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
