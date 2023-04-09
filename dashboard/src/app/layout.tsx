import './global.css'

export default function RootLayout({ children}: { children: React.ReactNode }) {


	return (
		<html lang="en">
			<body style={{maxWidth: '100vw', overflowX: 'hidden'}}>
				{children}
			</body>
		</html>
	)
}
