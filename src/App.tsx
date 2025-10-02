import {useState} from "react";
//import {invoke} from "@tauri-apps/api/core";
import {ThemeProvider} from '@mui/material/styles';
// import {Box, Container, Grid, Stack} from "@mui/material";


import "./App.scss";
import {theme} from "./Theme.ts";
import Header from "./components/header/Header.tsx";
// import MonthSelector from "./components/MonthSelector.tsx";
import HamburgerMenu from "./components/hamburguer_menu/HamburgerMenu.tsx";
// import Footer from "./components/Footer.tsx";
// import QuickPurchaseMenu from "./components/QuickPurchaseMenu.tsx";

import {Stack} from "@mui/material";
import {BrowserRouter} from "react-router";
import Main from "./components/main/Main.tsx";

function App() {
    // const [greetMsg, setGreetMsg] = useState("");
    // const [name, setName] = useState("");
    const [drawerOpen, setDrawerOpen] = useState(false);
    //
    // async function greet() {
    //     setGreetMsg(await invoke("greet", {name}));
    // }

    return (
        <BrowserRouter>
            <ThemeProvider theme={theme}>
                <Stack>
                    <Header setDrawerOpen={setDrawerOpen}/>
                    <Main/>
                </Stack>
                <HamburgerMenu drawerOpen={drawerOpen} setDrawerOpen={setDrawerOpen}/>
            </ThemeProvider>
        </BrowserRouter>
    );
}

export default App;