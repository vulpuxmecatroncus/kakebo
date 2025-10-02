//import React from 'react';

import {Box, Container, Grid, Stack} from "@mui/material";
import QuickPurchaseMenu from "../QuickPurchaseMenu.tsx";
import Footer from "../Footer.tsx";
import MonthSelector from "../MonthSelector.tsx";
import {Route, Routes} from "react-router";
import Items from "./items/Items.tsx";


const Main = () => {
    return (
        <Grid container flex={1} height={{xs: "80vh", md: "90%"}}>
            <Grid size={{xs: 10, md: 11}}>
                <Stack direction="column" p={2} gap={2} height={"95%"}>
                    <Container sx={{
                        display: "flex",
                        justifyContent: "center",
                        alignItems: "center",
                        height: "100%"
                    }}>
                        <Routes>
                            <Route path="/" element={<>Main Content</>}/>
                            <Route path="/items" element={<Items/>}/>
                        </Routes>
                    </Container>
                    <Box mt="auto" style={{textAlign: 'center'}}>
                        <QuickPurchaseMenu/>
                    </Box>
                    <Footer/>
                </Stack>
            </Grid>
            <Grid size={{xs: 2, md: 1}} display="flex" justifyContent="flex-end" height={"100%"}>
                <MonthSelector/>
            </Grid>
        </Grid>
    );
};

export default Main;