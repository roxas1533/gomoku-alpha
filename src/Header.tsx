import "@mantine/core/styles.css";
import { ActionIcon, Anchor } from "@mantine/core";
import "./App.css";
import { FaGithub } from "react-icons/fa";

function Header() {
	return (
		<header className="app-header">
			<div className="header">
				<div className="prefix" />
				<div>ミニマックス法</div>
				<Anchor
					href="https://github.com/roxas1533/gomoku-alpha"
					target="_blank"
					className="github-link suffix"
					aria-label="View source code on GitHub"
				>
					<ActionIcon size="lg" radius="md" variant="outline">
						<FaGithub size={24} />
					</ActionIcon>
				</Anchor>
			</div>
		</header>
	);
}

export default Header;
