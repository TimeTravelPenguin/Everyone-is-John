﻿#region Title Header

// Name: Phillip Smith
// 
// Solution: EveryoneIsJohnTracker
// Project: EveryoneIsJohnTracker
// File Name: VoiceModel.cs
// 
// Current Data:
// 2019-12-11 7:02 PM
// 
// Creation Date:
// 2019-09-27 9:09 AM

#endregion

using System.Collections.ObjectModel;
using System.Collections.Specialized;
using System.Linq;
using EveryoneIsJohnTracker.Base;
using EveryoneIsJohnTracker.Models.OutputLoggers;
using Newtonsoft.Json;

namespace EveryoneIsJohnTracker.Models
{
    internal class VoiceModel : PropertyChangedBase
    {
        private ILogger _logger;
        private string _name;
        private ObsessionModel _obsession = new ObsessionModel();
        private ObservableCollection<SkillModel> _skills = new ObservableCollection<SkillModel>();
        private int _willpower;

        [JsonIgnore]
        public ILogger Logger
        {
            get => _logger;
            set
            {
                SetValue(ref _logger, value);
                _obsession.Logger = value;
            }
        }

        public string Name
        {
            get => _name;
            set
            {
                var oldName = Name;
                SetValue(ref _name, value);

                if (oldName != value)
                {
                    Logger.LogNameChanged(oldName, value);
                }

                Obsession.VoiceName = Name;
            }
        }

        public int Willpower
        {
            get => _willpower;
            set
            {
                var oldValue = Willpower;
                SetValue(ref _willpower, value);

                if (oldValue != value)
                {
                    Logger.LogWillPowerChanged(Name, oldValue, value);
                }
            }
        }

        public ObservableCollection<SkillModel> Skills
        {
            get => _skills;
            set => SetValue(ref _skills, value);
        }

        public string SkillsAsString => string.Join(", ", Skills.Select(x => x.Name).ToArray());

        public ObsessionModel Obsession
        {
            get => _obsession;
            set => SetValue(ref _obsession, value);
        }

        public VoiceModel()
        {
            Logger = LogFactory.NewLogger(LoggerType.NullLogger);
            Obsession.Logger = LogFactory.NewLogger(LoggerType.NullLogger);
        }

        public VoiceModel(ILogger logger)
        {
            Logger = logger;
            Obsession.Logger = logger;
            Willpower = 7;
            Skills.CollectionChanged += SkillsCollectionChanged;
        }

        public VoiceModel(VoiceModel voice)
        {
            Logger = LogFactory.NewLogger(LoggerType.NullLogger);
            Obsession.Logger = LogFactory.NewLogger(LoggerType.NullLogger);

            Obsession.Name = voice.Obsession.Name;
            Obsession.Level = voice.Obsession.Level;
            Obsession.Points = voice.Obsession.Points;
            Obsession.VoiceName = voice.Name;

            Name = voice.Name;
            Willpower = voice.Willpower;

            Skills.Clear();
            foreach (var skill in voice.Skills)
            {
                Skills.Add(new SkillModel
                {
                    Name = skill.Name
                });
            }

            Logger = voice.Logger;
            Obsession.Logger = voice.Logger;
        }

        // TODO
        private static void SkillsCollectionChanged(object sender, NotifyCollectionChangedEventArgs e)
        {
            //throw new NotImplementedException();
        }

        internal void Clear()
        {
            Name = "";
            Willpower = 7;
            Skills.Clear();
            Obsession.Name = "";
            Obsession.Level = 1;
        }
    }
}