// "use client"
import Link from 'next/link'
import style from '@/styles/layout.module.scss';
import './global.css'

export default function RootLayout({ children }: { children: React.ReactNode }) {
	return (
		<html lang="en">
			<body>
				{children}
			</body>
		</html>
	)
}
