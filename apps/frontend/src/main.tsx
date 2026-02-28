import { createRoot } from "react-dom/client"
import "./index.css"
import { Feed } from "./routes"
import { BrowserRouter, Route, Routes } from "react-router"

function App() {
	return (
		<BrowserRouter>
			<Routes>
				<Route path="/feed" element={<Feed />} />
			</Routes>
		</BrowserRouter>
	)
}

createRoot(document.getElementById("root")!).render(<App />)
