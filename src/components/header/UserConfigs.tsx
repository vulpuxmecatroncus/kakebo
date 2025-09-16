//import React from 'react';
import {Box, SpeedDial, SpeedDialAction} from "@mui/material";
import SettingsIcon from "@mui/icons-material/Settings";
import AccountCircle from "@mui/icons-material/AccountCircle";
import PrintIcon from "@mui/icons-material/Print";
import LanguageIcon from "@mui/icons-material/Language";
import SaveIcon from "@mui/icons-material/Save";
import {Dispatch, SetStateAction} from "react";

const UserConfigs = ({setOpen}: { setOpen: Dispatch<SetStateAction<boolean>> }) => {
    const actions = [
        {icon: <AccountCircle/>, name: 'Copy'},
        {icon: <SaveIcon/>, name: 'Save'},
        {icon: <PrintIcon/>, name: 'Print'},
        {icon: <LanguageIcon/>, name: 'Language'},
    ];
    return (
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
                            onClick={() => action.name === 'Language' && setOpen(true)}
                        />
                    ))}
                </SpeedDial>
            </Box>
        </Box>
    );
};

export default UserConfigs;