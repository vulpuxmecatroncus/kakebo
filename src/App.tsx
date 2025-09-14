import {useState} from "react";
//import {invoke} from "@tauri-apps/api/core";
import MenuIcon from "@mui/icons-material/Menu";
import {ThemeProvider} from '@mui/material/styles';
import {
    Box,
    Button,
    Drawer,
    Grid,
    IconButton,
    List,
    ListItemButton,Tab,
    Tabs,
    Typography
} from "@mui/material";

import "./App.scss";
import {AccountCircle, Home} from "@mui/icons-material";
import {theme} from "./Theme.ts";

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
                <Box component="header" boxShadow={1}>
                    <Box display="flex" alignItems="center" justifyContent="space-between" p={2}>
                        <IconButton edge="start" color="inherit" aria-label="menu" onClick={() => setDrawerOpen(true)}>
                            <MenuIcon/>
                        </IconButton>
                        <IconButton edge="start" color="inherit" aria-label="home">
                            <Home/>
                        </IconButton>
                        <Typography variant="h5" sx={{flexGrow: 1, textAlign: "center"}}>
                            Welcome to Kakebo
                        </Typography>
                        <IconButton color="inherit">
                            <AccountCircle/>
                        </IconButton>
                    </Box>
                </Box>
                <Box display="flex" height="90vh">
                    {/* Hamburger Drawer */}
                    <Drawer anchor="left" open={drawerOpen} onClose={() => setDrawerOpen(false)}>
                        <List>
                            <ListItemButton>Menu Item 1</ListItemButton>
                            <ListItemButton>Menu Item 2</ListItemButton>
                            <ListItemButton>Menu Item 3</ListItemButton>
                        </List>
                    </Drawer>
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
                    <Tabs orientation="vertical" variant="scrollable" value={0}
                          sx={{
                              borderLeft: 1,
                              borderColor: 'divider',
                              // minWidth: 150,
                              '& .MuiTab-root': {writingMode: 'sideways-rl', textOrientation: 'sideways'}
                          }}>
                        {[...Array(12)].map((_, i) => (
                            <Tab key={i} label={`Tab ${i + 1}`}/>
                        ))}
                    </Tabs>
                </Box>
            </Box>
        </ThemeProvider>
    );
}

export default App;