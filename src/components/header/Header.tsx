import React, {Dispatch, SetStateAction} from 'react';

import {AccountCircle, Home} from "@mui/icons-material";
import {Box, IconButton, SpeedDial, SpeedDialAction, Typography} from "@mui/material";
//SpeedDialIcon,
import MenuIcon from "@mui/icons-material/Menu";
import {useTranslation} from 'react-i18next';
import SaveIcon from '@mui/icons-material/Save';
import PrintIcon from '@mui/icons-material/Print';
//import ShareIcon from '@mui/icons-material/Share';
import SettingsIcon from '@mui/icons-material/Settings';
import LanguageIcon from '@mui/icons-material/Language';
import {ChangeLanguage} from "./changeLanguage/ChangeLanguage.tsx";

const Header = ({setDrawerOpen}: { setDrawerOpen: Dispatch<SetStateAction<boolean>> }) => {

    const [open, setOpen] = React.useState(false);
    const actions = [
        {icon: <AccountCircle/>, name: 'Copy'},
        {icon: <SaveIcon/>, name: 'Save'},
        {icon: <PrintIcon/>, name: 'Print'},
        {icon: <LanguageIcon/>, name: 'Language'},
    ];

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
                    {useTranslation().t('title')}
                </Typography>
                <Box display="flex" alignItems="center" position="relative" height={56}>
                    <Box position="absolute" right={0} top={0} zIndex={2}>
                        <SpeedDial
                            ariaLabel="SpeedDial basic example"
                            icon={<SettingsIcon/>}
                            direction="down"
                        >
                            {actions.map((action) => (
                                <SpeedDialAction
                                    key={action.name}
                                    icon={action.icon}
                                    slotProps={{
                                        tooltip: {
                                            title: action.name,
                                        },
                                    }}
                                    onClick={action.name === 'Language' ? () => setOpen(true) : undefined}
                                />
                            ))}
                        </SpeedDial>
                    </Box>
                </Box>
            </Box>
            <ChangeLanguage open={open} setOpen={setOpen}/>
        </Box>
    );
};

export default Header;