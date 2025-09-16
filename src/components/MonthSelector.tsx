//import React from 'react';
import {Tab, Tabs} from "@mui/material";
import {useTranslation} from 'react-i18next';
import {useState} from "react";

const MonthSelector = () => {

    const {t} = useTranslation();
    const months = t('months', {returnObjects: true}) as string[];
    const [selectedTab, setSelectedTab] = useState<number | false>(false);

    return (
        <Tabs orientation="vertical" variant="scrollable"
              value={selectedTab === false ? false : selectedTab}
              onChange={(_, newValue) => {
                  setSelectedTab(prev =>
                      prev === newValue ? false : newValue
                  );
              }}
              sx={{
                  borderLeft: 2,
                  borderColor: 'divider',
                  // minWidth: 150,
                  '& .MuiTab-root': {writingMode: 'sideways-rl', textOrientation: 'sideways'}
              }}>
            {months.map((month, i) => (
                <Tab key={i} label={month}/>
            ))}
        </Tabs>
    );
};

export default MonthSelector;