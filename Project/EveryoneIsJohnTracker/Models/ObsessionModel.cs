﻿using System.Windows;
using EveryoneIsJohnTracker.Base;
using EveryoneIsJohnTracker.Models.OutputLoggers;

namespace EveryoneIsJohnTracker.Models
{
    internal class ObsessionModel : PropertyChangedBase
    {
        private int _level;
        private string _name;
        private int _points;
        private string _voiceName;
        public IOutputLogger Logger { get; set; }

        public string VoiceName
        {
            get => _voiceName;
            set => SetValue(ref _voiceName, value);
        }

        public int Level
        {
            get => _level;
            set
            {
                var oldValue = Level;
                CheckLevelValue(ref value);
                SetValue(ref _level, value);

                if (oldValue != value)
                {
                    Logger.LogObsessionLevelChanged(VoiceName, Name, oldValue, value);
                }
            }
        }

        public string Name
        {
            get => _name;
            set
            {
                var oldValue = Name;
                SetValue(ref _name, value);

                if (oldValue != value)
                {
                    Logger.LogObsessionNameChanged(VoiceName, oldValue, value);
                }
            }
        }

        public int Points
        {
            get => _points;
            set
            {
                var oldValue = Points;
                SetValue(ref _points, value);

                if (oldValue != value)
                {
                    Logger.LogObsessionPointsChanged(VoiceName, oldValue, value);
                }
            }
        }

        public ObsessionModel(IOutputLogger logger)
        {
            Logger = logger;
        }

        public ObsessionModel()
        {
        }

        internal void CheckLevelValue(ref int level)
        {
            if (level >= 1 && level <= 3)
            {
                return;
            }

            level = 1;

            // TODO: Get rid of this garbo
            MessageBox.Show("Level value must be between 1 and 3", "Error assigning Obsession Level");
        }
    }
}