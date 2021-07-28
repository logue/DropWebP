﻿namespace DropWebP.Utility
{
    using MahApps.Metro.Controls;
    using Prism.Regions;
    using System.Collections.Specialized;
    using System.Windows;

    /// <summary>
    /// Defines the <see cref="FlyoutsControlRegionAdapter" />.
    /// </summary>
    internal class FlyoutsControlRegionAdapter : RegionAdapterBase<FlyoutsControl>
    {
        /// <summary>
        /// Initializes a new instance of the <see cref="FlyoutsControlRegionAdapter"/> class.
        /// </summary>
        /// <param name="factory">.</param>
        public FlyoutsControlRegionAdapter(IRegionBehaviorFactory factory)
            : base(factory)
        {
        }

        /// <summary>
        /// The Adapt.
        /// </summary>
        /// <param name="region">The region<see cref="IRegion"/>.</param>
        /// <param name="regionTarget">The regionTarget<see cref="FlyoutsControl"/>.</param>
        protected override void Adapt(IRegion region, FlyoutsControl regionTarget)
        {
            region.ActiveViews.CollectionChanged += (s, e) =>
            {
                if (e.Action != NotifyCollectionChangedAction.Add)
                {
                    return;
                }
                foreach (FrameworkElement element in e.NewItems)
                {
                    Flyout flyout = new Flyout();
                    flyout.Content = element;
                    flyout.DataContext = element.DataContext;
                    _ = regionTarget.Items.Add(flyout);
                }
            };
        }

        /// <summary>
        /// The CreateRegion.
        /// </summary>
        /// <returns>The <see cref="IRegion"/>.</returns>
        protected override IRegion CreateRegion()
        {
            return new AllActiveRegion();
        }
    }
}
