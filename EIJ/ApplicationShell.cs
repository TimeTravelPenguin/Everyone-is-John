﻿#region Title Header

// Name: Phillip Smith
// 
// Solution: EIJ
// Project: EIJ
// File Name: ApplicationShell.cs
// 
// Current Data:
// 2021-02-15 10:58 PM
// 
// Creation Date:
// 2021-02-13 7:42 PM

#endregion

using System.Windows;
using EIJ.Debug;
using EIJ.Models.Logging;
using EIJ.ViewModels.Pages;
using EIJ.ViewModels.UserControls;
using EIJ.ViewModels.Windows;
using EIJ.Views.Pages;
using EIJ.Views.UserControls;
using EIJ.Views.Windows;

namespace EIJ
{
  internal static class ApplicationShell
  {
    private static readonly ILogger Logger = new GameLogger();

    public static void Start()
    {
      var mainWindow = NewMainWindow();
      mainWindow.Show();
    }

    private static Window NewMainWindow()
    {
      var window = new ViewBase();

      var loggerControlView = new LoggerControlView
      {
        DataContext = new LoggerControlViewModel(Logger)
      };

      var horizontalSplitPage = new HorizontalSplitPageView
      {
        DataContext =
          new HorizontalSplitPageViewModel(new DebugPageView {DataContext = new DebugPageViewModel {Logger = Logger}},
            loggerControlView)
      };

      var mainPage = new ToolbarPageView
      {
        DataContext = new ToolbarPageViewModel(horizontalSplitPage)
      };

      var windowVm = new ViewModelBase(window)
      {
        WindowTitle = "Everyone is John Tracker",
        CurrentPage = mainPage
      };

      window.DataContext = windowVm;

      return window;
    }
  }
}