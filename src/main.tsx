import { StrictMode } from "react";
import "@mantine/core/styles.css";
import { createRoot } from "react-dom/client";
import "./index.css";
import { MantineProvider } from "@mantine/core";
import App from "./App.tsx";

createRoot(document.getElementById("root")!).render(
	<StrictMode>
		<MantineProvider defaultColorScheme="dark">
			<App />
		</MantineProvider>
	</StrictMode>,
);
