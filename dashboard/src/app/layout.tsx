import NavBar from '@/components/client/navbar'
import './global.css'

export default function RootLayout({ children }: { children: React.ReactNode }) {
	return (
		<html lang="en">
			<body style={{ height: '100vh', maxWidth: '100vw', overflowX: 'hidden', display: 'flex', flexDirection: 'column' }}>
				<NavBar />
				{children}
			</body>
		</html>
	)
}
