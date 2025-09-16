import {useState} from "react";
//import {invoke} from "@tauri-apps/api/core";
import {ThemeProvider} from '@mui/material/styles';
import {Box, Button, Grid, Typography} from "@mui/material";


import "./App.scss";
import {theme} from "./Theme.ts";
import Header from "./components/header/Header.tsx";
import MonthSelector from "./components/MonthSelector.tsx";
import HamburgerMenu from "./components/HamburgerMenu.tsx";

function App() {
    // const [greetMsg, setGreetMsg] = useState("");
    // const [name, setName] = useState("");
    const [drawerOpen, setDrawerOpen] = useState(false);
    //
    // async function greet() {
    //     setGreetMsg(await invoke("greet", {name}));
    // }

    return (
        <ThemeProvider theme={theme}>
            <Box component="main">
                <Header setDrawerOpen={setDrawerOpen}/>
                <Box display="flex" height="90vh">
                    {/* Hamburger Drawer */}
                    <HamburgerMenu drawerOpen={drawerOpen} setDrawerOpen={setDrawerOpen}/>
                    {/* Main Content */}
                    <Box flex={1} display="flex" flexDirection="column" justifyContent="space-between" p={2}>
                        <Box flex={1}>
                            <Typography variant="body1">Main Content Area</Typography>
                        </Box>
                        <Box>
                            <Grid container spacing={2} mb={1}>
                                <Grid size={4}><Button variant="contained" fullWidth>Button 1</Button></Grid>
                                <Grid size={4}><Button variant="contained" fullWidth>Button 2</Button></Grid>
                                <Grid size={4}><Button variant="contained" fullWidth>Button 3</Button></Grid>
                            </Grid>
                            <Grid container spacing={2}>
                                <Grid size={4}><Button variant="contained" fullWidth>Button 4</Button></Grid>
                                <Grid size={4}><Button variant="contained" fullWidth>Button 5</Button></Grid>
                                <Grid size={4}><Button variant="contained" fullWidth>Button 6</Button></Grid>
                            </Grid>
                        </Box>
                    </Box>
                    {/* Vertical Tabs */}
                    <MonthSelector/>
                </Box>
            </Box>
        </ThemeProvider>
    );
}

export default App;