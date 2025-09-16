import {Box, SpeedDial, SpeedDialAction} from "@mui/material";
import SettingsIcon from "@mui/icons-material/Settings";
import AccountCircle from "@mui/icons-material/AccountCircle";
import PrintIcon from "@mui/icons-material/Print";
import LanguageIcon from "@mui/icons-material/Language";
import SaveIcon from "@mui/icons-material/Save";
import {Dispatch, SetStateAction, useState} from "react";

const UserConfigs = ({openChangeLanguage}: { openChangeLanguage: Dispatch<SetStateAction<boolean>> }) => {
    const [openSpeed, setOpenSpeed] = useState<boolean>(false);
    const actions = [
        {icon: <AccountCircle/>, name: 'Copy'},
        {icon: <SaveIcon/>, name: 'Save'},
        {icon: <PrintIcon/>, name: 'Print'},
        {icon: <LanguageIcon/>, name: 'Language'},
    ];
    return (
        <Box display="flex" alignItems="center" position="relative" height={56}>
            <Box position="absolute" right={0} top={0}>
                <SpeedDial
                    ariaLabel="SpeedDial basic example"
                    icon={<SettingsIcon/>}
                    direction="down"
                    open={openSpeed}
                    onOpen={() => setOpenSpeed(true)}
                    onClose={(_event, reason) => {
                        if (reason !== "mouseLeave") {
                            setOpenSpeed(false);
                        }
                    }
                    }
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
                            onClick={() => {
                                setOpenSpeed(false);
                                action.name === 'Language' && openChangeLanguage(true);
                            }
                            }
                        />
                    ))}
                </SpeedDial>
            </Box>
        </Box>
    );
};

export default UserConfigs;