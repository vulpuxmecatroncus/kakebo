import {useState} from "react";
//import {invoke} from "@tauri-apps/api/core";
import {ThemeProvider} from '@mui/material/styles';
import {Box, Container, Grid, Stack} from "@mui/material";


import "./App.scss";
import {theme} from "./Theme.ts";
import Header from "./components/header/Header.tsx";
import MonthSelector from "./components/MonthSelector.tsx";
import HamburgerMenu from "./components/HamburgerMenu.tsx";
import Footer from "./components/Footer.tsx";

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
            <Stack>
                <Header setDrawerOpen={setDrawerOpen}/>
                <Grid container flex={1} height={{xs: "80vh", md: "90%"}}>
                    <Grid size={{xs: 10, md: 11}}>
                        <Stack direction="column" p={2} gap={2} height={"95%"}>
                            <Container sx={{
                                display: "flex",
                                justifyContent: "center",
                                alignItems: "center",
                                height: "100%"
                            }}>Content</Container>
                            <Box mt="auto" style={{textAlign: 'center'}}>Second Item</Box>
                            <Footer/>
                        </Stack>
                    </Grid>
                    <Grid size={{xs: 2, md: 1}} display="flex" justifyContent="flex-end" height={"100%"}>
                        <MonthSelector/>
                    </Grid>
                </Grid>
            </Stack>

            {/*<Box component="main">*/}
            {/*     <Box display="flex">*/}
            {/*        /!* Hamburger Drawer *!/*/}
            {/*        /!* Main Content *!/*/}
            {/*        <Box flex={1} display="flex" flexDirection="column" justifyContent="space-between" p={2}>*/}
            {/*            <Box flex={1}>*/}
            {/*                <Typography variant="body1">Main Content Area</Typography>*/}
            {/*            </Box>*/}
            {/*            <Box>*/}
            {/*                <Grid container spacing={2} mb={1}>*/}
            {/*                    <Grid size={4}><Button variant="contained" fullWidth>Button 1</Button></Grid>*/}
            {/*                    <Grid size={4}><Button variant="contained" fullWidth>Button 2</Button></Grid>*/}
            {/*                    <Grid size={4}><Button variant="contained" fullWidth>Button 3</Button></Grid>*/}
            {/*                </Grid>*/}
            {/*                <Grid container spacing={2}>*/}
            {/*                    <Grid size={4}><Button variant="contained" fullWidth>Button 4</Button></Grid>*/}
            {/*                    <Grid size={4}><Button variant="contained" fullWidth>Button 5</Button></Grid>*/}
            {/*                    <Grid size={4}><Button variant="contained" fullWidth>Button 6</Button></Grid>*/}
            {/*                </Grid>*/}
            {/*            </Box>*/}
            {/*        </Box>*/}
            {/*        /!* Vertical Tabs *!/*/}

            {/*    </Box>*/}
            {/*</Box>*/}
            <HamburgerMenu drawerOpen={drawerOpen} setDrawerOpen={setDrawerOpen}/>
        </ThemeProvider>
    );
}

export default App;