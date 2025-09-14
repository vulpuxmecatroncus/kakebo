import {Dispatch, SetStateAction} from 'react';

import {AccountCircle, Home} from "@mui/icons-material";
import {Box, IconButton, Typography} from "@mui/material";
import MenuIcon from "@mui/icons-material/Menu";

const Header = ({setDrawerOpen}: { setDrawerOpen: Dispatch<SetStateAction<boolean>> }) => {
    return (
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
    );
};

export default Header;