import NavBar from '@/components/client/navbar'
import './global.css'

export default function RootLayout({ children }: { children: React.ReactNode }) {
	return (
		<html lang="en">
			<body className={'overflow-x-hidden flex-col flex h-screen p-0 m-0'}>
				<NavBar />
				{children}
			</body>
		</html>
	)
}
